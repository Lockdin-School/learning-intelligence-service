use crate::core::progress::models::ProgressHistory::NewProgressHistory;
use crate::core::progress::models::ProgressSignal::ConceptProgressSignal;
use crate::core::progress::repository::interface::ProgressRepository::{
    ConceptHierarchyRef, ProgressRepository, ProgressRepositoryError,
};
use crate::infrastructure::InternalEventBus::Event;
use actix_web::{HttpResponse, ResponseError};
use chrono::{DateTime, Duration, Utc};
use rust_decimal::prelude::ToPrimitive;
use serde::Serialize;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;
use crate::core::progress::models::SubjectProgress::SubjectProgress;
use crate::core::progress::models::TopicProgress::TopicProgress;

#[derive(Debug, Error)]
pub enum ProgressError {
    #[error(transparent)]
    Repository(#[from] ProgressRepositoryError),
    #[error("transaction error: {0}")]
    Transaction(#[from] sqlx::Error),
}

impl ResponseError for ProgressError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::Repository(_) | Self::Transaction(_) => {
                HttpResponse::InternalServerError().json(self.to_string())
            }
        }
    }
}

#[derive(Debug, Clone, Serialize)]

pub enum MasteryStatus {
    InsufficientData,
    AtRisk,
    Developing,
    Proficient,
}

#[derive(Debug, Clone, Serialize)]
pub enum Trend {
    New,
    Improving,
    Declining,
    Stable,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConceptProgress {
    pub concept_id: Uuid,
    pub mastery: f64,
    pub questions_answered: i32,
    pub status: MasteryStatus,
    pub trend: Trend,
    pub delta_30d: Option<f64>,
    pub last_attempted_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StudentProgressSummary {
    pub student_id: Uuid,
    pub concepts: Vec<ConceptProgress>,
}

// Tuning constants — placeholders, not pedagogy. See the note below.
const MIN_SAMPLE_SIZE: i32 = 3;
const AT_RISK_CEILING: f64 = 0.5;
const PROFICIENT_FLOOR: f64 = 0.8;
const TREND_NOISE_BAND: f64 = 0.05;

#[derive(Clone)]
pub struct ProgressService {
    repo: Arc<dyn ProgressRepository>,
    pool: PgPool,
}

impl ProgressService {
    pub fn new(repo: Arc<dyn ProgressRepository>, pool: PgPool) -> Self {
        Self { repo, pool }
    }

    pub async fn events_handler(&self, mut receiver: tokio::sync::broadcast::Receiver<Event>) {
        log::info!(
            "progress.service | service | events_handler | started | \"Listening for events\" |"
        );

        while let Ok(event) = receiver.recv().await {
            if let Event::ConceptProgressSignalsGenerated(signals) = event.clone() {
                log::info!(
                    "progress.service | service | events_handler | signals received | \"processing signals\" |"
                );
                for signal in signals {
                    if let Err(err) = self.process_signal(signal).await {
                        tracing::error!(?err, "failed to process concept progress signal");
                    }
                }
            }
        }
    }

    /// What a new answer does to this student's accumulated concept state.
    pub async fn process_signal(&self, signal: ConceptProgressSignal) -> Result<(), ProgressError> {
        log::info!(
            "service.progress.process_signal | service | process_signal | Started | signal: {:?}",
            signal
        );

        let mut tx = self.pool.begin().await?;


        let previous = self
            .repo
            .get_current_progress_for_concept(signal.student_id, signal.concept_id)
            .await?;

        let (questions_answered, questions_correct) = match previous {
            Some(row) => (
                row.questions_answered + 1,
                row.questions_correct + i32::from(signal.correct),
            ),
            None => (1, i32::from(signal.correct)),
        };

        let entry = NewProgressHistory {
            student_id: signal.student_id,
            concept_id: signal.concept_id,
            observation_id: signal.source_observation_id,
            questions_answered,
            questions_correct,
        };

        let hierarchy = ConceptHierarchyRef {
            concept_id: signal.concept_id,
            topic_id: signal.topic_id,
            subject_id: signal.subject_id,
        };

        self.repo.record_progress(&mut tx, entry, hierarchy).await?;
        tx.commit().await?;

        Ok(())
    }

    /// What accumulated state means for the student — current standing plus
    /// a 30-day trend, per concept.
    pub async fn get_student_progress(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<SubjectProgress>, ProgressError> {
        let rows = self.repo.get_current_progress_with_hierarchy(student_id).await?;

        let cutoff: DateTime<Utc> = Utc::now() - Duration::days(30);
        let past = self.repo.get_progress_as_of(student_id, cutoff).await?;
        let past_by_concept: HashMap<Uuid, f64> = past
            .into_iter()
            .map(|row| (row.concept_id, row.accuracy.to_f64().unwrap_or(0.0)))
            .collect();

        struct TopicAccum {
            subject_id: Uuid,
            total_answered: i32,
            total_correct: i32,
            last_attempted_at: DateTime<Utc>,
            concepts: Vec<ConceptProgress>,
        }

        let mut topics: HashMap<Uuid, TopicAccum> = HashMap::new();

        for row in rows {
            let mastery = row.accuracy.to_f64().unwrap_or(0.0);
            let status = classify_status(mastery, row.questions_answered);

            let (trend, delta_30d) = match past_by_concept.get(&row.concept_id) {
                Some(&past_mastery) => {
                    let delta = mastery - past_mastery;
                    let direction = if delta > TREND_NOISE_BAND {
                        Trend::Improving
                    } else if delta < -TREND_NOISE_BAND {
                        Trend::Declining
                    } else {
                        Trend::Stable
                    };
                    (direction, Some(delta))
                }
                None => (Trend::New, None),
            };

            let concept_progress = ConceptProgress {
                concept_id: row.concept_id,
                mastery,
                questions_answered: row.questions_answered,
                status,
                trend,
                delta_30d,
                last_attempted_at: row.last_attempted_at,
            };

            let accum = topics.entry(row.topic_id).or_insert_with(|| TopicAccum {
                subject_id: row.subject_id,
                total_answered: 0,
                total_correct: 0,
                last_attempted_at: row.last_attempted_at,
                concepts: Vec::new(),
            });

            accum.total_answered += row.questions_answered;
            accum.total_correct += row.questions_correct;
            accum.last_attempted_at = accum.last_attempted_at.max(row.last_attempted_at);
            accum.concepts.push(concept_progress);
        }

        struct SubjectAccum {
            total_answered: i32,
            total_correct: i32,
            last_attempted_at: DateTime<Utc>,
            topics: Vec<TopicProgress>,
        }

        let mut subjects: HashMap<Uuid, SubjectAccum> = HashMap::new();

        for (topic_id, accum) in topics {
            let TopicAccum { subject_id, total_answered, total_correct, last_attempted_at, concepts } = accum;

            let topic_progress = TopicProgress {
                topic_id,
                mastery: if total_answered == 0 { 0.0 } else { total_correct as f64 / total_answered as f64 },
                questions_answered: total_answered,
                last_attempted_at,
                concepts,
            };

            let subj = subjects.entry(subject_id).or_insert_with(|| SubjectAccum {
                total_answered: 0,
                total_correct: 0,
                last_attempted_at,
                topics: Vec::new(),
            });

            subj.total_answered += total_answered;
            subj.total_correct += total_correct;
            subj.last_attempted_at = subj.last_attempted_at.max(last_attempted_at);
            subj.topics.push(topic_progress);
        }

        let result = subjects
            .into_iter()
            .map(|(subject_id, accum)| SubjectProgress {
                subject_id,
                mastery: if accum.total_answered == 0 { 0.0 } else { accum.total_correct as f64 / accum.total_answered as f64 },
                questions_answered: accum.total_answered,
                last_attempted_at: accum.last_attempted_at,
                topics: accum.topics,
            })
            .collect();

        Ok(result)
    }
}

fn classify_status(mastery: f64, questions_answered: i32) -> MasteryStatus {
    if questions_answered < MIN_SAMPLE_SIZE {
        return MasteryStatus::InsufficientData;
    }
    if mastery < AT_RISK_CEILING {
        MasteryStatus::AtRisk
    } else if mastery < PROFICIENT_FLOOR {
        MasteryStatus::Developing
    } else {
        MasteryStatus::Proficient
    }
}

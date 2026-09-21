use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct QuizAttemptGradedData {
    pub quiz_id: Uuid,
    pub attempt_id: Uuid,
    pub subject_id: Uuid,
    pub topic_id: Uuid,

    pub score: i32,
    pub passing_score: i32,
    pub max_score: i32,
    pub percentage: f64,

    pub items: Vec<QuizItemObservation>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct QuizItemObservation {
    pub question_id: Uuid,
    pub correct: bool,
    pub concept_id: Uuid,
    pub answered_at: DateTime<Utc>,
}

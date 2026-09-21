use crate::core::observations::models::Observation::Observation;
use crate::core::observations::payloads::QuizAttemptedGradedData::QuizAttemptGradedData;
use crate::core::progress::models::ProgressSignal::ConceptProgressSignal;
use crate::core::transformation::errors::TransformationError::TransformationError;
use crate::infrastructure::InternalEventBus::{Event, EventBus};
use actix_web::web::Data;
use tokio::sync::broadcast::error::SendError;

pub struct QAGTransformer {
    event_bus: Data<EventBus>,
}

impl QAGTransformer {
    pub fn new(event_bus: Data<EventBus>) -> Self {
        Self { event_bus }
    }

    pub async fn events_handler(&self, mut receiver: tokio::sync::broadcast::Receiver<Event>) {
        log::info!(
            "qag_transformer.events.listen | transformer | events_handler | started | \"Listening for events\" |"
        );

        while let Ok(event) = receiver.recv().await {
            if let Event::QuizAttemptGraded(obs) = event.clone() {
                log::info!(
                    "qag_transformer.events.listen | transformer | events_handler | in-progress | \"Received Event: QuizAttemptGraded.\" |"
                );

                // TRANSFORM
                match self.transform(&obs) {
                    Ok(signals) => {
                        log::info!(
                            "qag_transformer.events.listen | transformer | events_handler | success | \"Transformed event: QuizAttemptGraded. Signals generated\" |"
                        );

                        // Publish signals
                        let _ = self.publish_signals(signals);
                    }
                    Err(e) => {
                        log::error!(
                            "qag_transformer.events.listen | transformer | events_handler | failed | \"Failed to transform event: QuizAttemptGraded\" | error=\"{}\"",
                            e
                        );
                    }
                }
            }
        }
    }

    pub fn transform(
        &self,
        observation: &Observation,
    ) -> Result<Vec<ConceptProgressSignal>, TransformationError> {
        let data: QuizAttemptGradedData = serde_json::from_value(observation.data.clone())
            .map_err(TransformationError::InvalidData)?;

        let signals = data
            .items
            .into_iter()
            .map(|item| ConceptProgressSignal {
                student_id: observation.student_id,
                concept_id: item.concept_id,
                topic_id: data.topic_id,
                subject_id: data.subject_id,
                correct: item.correct,
                source_observation_id: observation.id,
                occurred_at: item.answered_at,
            })
            .collect();

        Ok(signals)
    }

    pub fn publish_signals(
        &self,
        signals: Vec<ConceptProgressSignal>,
    ) -> Result<usize, Box<SendError<Event>>> {
        match self
            .event_bus
            .send(Event::ConceptProgressSignalsGenerated(signals))
        {
            Ok(res) => {
                log::info!(
                    "qag_transformer.events.listen | transformer | events_handler | success | \"Published concept progress signals\""
                );
                Ok(res)
            }
            Err(error) => {
                log::error!(
                    "qag_transformer.events.listen | transformer | events_handler | error | \"Failed to publish concept progress signals\" | error: {:?}",
                    error
                );
                Err(Box::from(error))
            }
        }
    }
}

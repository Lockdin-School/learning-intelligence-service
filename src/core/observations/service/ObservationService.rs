use crate::core::observations::models::Observation::{Observation, ObservationNew};
use crate::core::observations::repository::ObservationRepository::ObservationRepository;
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use std::sync::Arc;
use actix_web::web::Data;
use tokio::sync::broadcast::error::SendError;
use uuid::Uuid;
use crate::infrastructure::InternalEventBus::{Event, EventBus};

#[derive(Debug, Display)]
pub enum ObservationServiceError {
    #[display("Database error")]
    Database(sqlx::Error),

    #[display("Observation not found")]
    NotFound,
}

impl ResponseError for ObservationServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::Database(_) => HttpResponse::InternalServerError().json(self.to_string()),

            Self::NotFound => HttpResponse::NotFound().json(self.to_string()),
        }
    }
}

pub struct ObservationService {
    repo: Arc<dyn ObservationRepository>,
    event_bus: Data<EventBus>,
}

impl ObservationService {
    pub fn new(repo: Arc<dyn ObservationRepository>, event_bus: Data<EventBus>) -> Self {
        Self { repo, event_bus }
    }

    pub fn publish_observation(
        &self,
        obs: &Observation,
    ) -> Result<usize, Box<SendError<Event>>> {
        match self
            .event_bus
            .send(Event::QuizAttemptGraded(obs.clone()))
        {
            Ok(res) => {
                log::info!(
                    "service.observation.publish | service | publish_observation | success | \"Published observation\""
                );
                Ok(res)
            }
            Err(error) => {
                log::error!(
                    "service.observation.publish | service | publish_observation | error | \"Failed to publish observation\" | error: {:?}",
                    error
                );
                Err(Box::from(error))
            }
        }
    }

    pub async fn create(
        &self,
        observation: &ObservationNew,
    ) -> Result<Observation, ObservationServiceError> {
        match self.repo.create(observation).await {
            Ok(observation) => {
                let _ = self.publish_observation(&observation);
                Ok(observation)
            },
            Err(error) => Err(ObservationServiceError::Database(error)),
        }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Observation, ObservationServiceError> {
        match self.repo.find_by_id(id).await {
            Ok(Some(observation)) => Ok(observation),
            Ok(None) => Err(ObservationServiceError::NotFound),
            Err(error) => Err(ObservationServiceError::Database(error)),
        }
    }

    pub async fn find_by_student_id(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<Observation>, ObservationServiceError> {
        match self.repo.find_by_student_id(student_id).await {
            Ok(observations) => Ok(observations),
            Err(error) => Err(ObservationServiceError::Database(error)),
        }
    }

    pub async fn exists_by_source_event(
        &self,
        source_service: &str,
        source_event_id: Uuid,
    ) -> Result<bool, ObservationServiceError> {
        match self
            .repo
            .exists_by_source_event(source_service, source_event_id)
            .await
        {
            Ok(exists) => Ok(exists),
            Err(error) => Err(ObservationServiceError::Database(error)),
        }
    }
}

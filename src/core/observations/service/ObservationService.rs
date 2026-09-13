use std::sync::Arc;
use uuid::Uuid;
use crate::core::observations::models::Observation::{Observation, ObservationNew};
use crate::core::observations::repository::ObservationRepository::ObservationRepository;
use derive_more::Display;
use actix_web::{HttpResponse, ResponseError};


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
            Self::Database(_) => {
                HttpResponse::InternalServerError().json(self.to_string())
            }

            Self::NotFound => {
                HttpResponse::NotFound().json(self.to_string())
            }
        }
    }
}

pub struct ObservationService {
    repo: Arc<dyn ObservationRepository>,
}

impl ObservationService {
    pub fn new(repo: Arc<dyn ObservationRepository>) -> Self {
        Self { repo }
    }

    pub async fn create(
        &self,
        observation: &ObservationNew,
    ) -> Result<Observation, ObservationServiceError> {
        match self.repo.create(observation).await {
            Ok(observation) => Ok(observation),
            Err(error) => Err(ObservationServiceError::Database(error)),
        }
    }

    pub async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Observation, ObservationServiceError> {
        match self.repo.find_by_id(id).await {
            Ok(Some(observation)) => Ok(observation),
            Ok(None) => Err(ObservationServiceError::NotFound),
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
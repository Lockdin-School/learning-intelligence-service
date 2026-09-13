use crate::core::observations::models::Observation::{Observation, ObservationNew};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ObservationRepository: Send + Sync {
    async fn create(&self, observation: &ObservationNew) -> sqlx::Result<Observation, sqlx::Error>;

    async fn find_by_id(&self, id: Uuid) -> sqlx::Result<Option<Observation>, sqlx::Error>;

    async fn exists_by_source_event(
        &self,
        source_service: &str,
        source_event_id: Uuid,
    ) -> sqlx::Result<bool, sqlx::Error>;
}

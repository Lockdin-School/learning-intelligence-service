use crate::core::telemetry::domain::TelemetryEvent::TelemetryEvent;
use crate::core::telemetry::domain::TelemetryEventType::TelemetryEventType;
use crate::core::telemetry::dto::TelemetryEventDbDTO::TelemetryEventDbDTO;

#[async_trait::async_trait]
pub trait TelemetryRepository {
    async fn save(&self, event: TelemetryEvent) -> Result<TelemetryEventDbDTO, sqlx::Error>;
    async fn get_all(&self) -> Result<Vec<TelemetryEvent>, sqlx::Error>;
    async fn get_by_type(&self, event_type: TelemetryEventType) -> Result<Vec<TelemetryEvent>, sqlx::Error>;
}
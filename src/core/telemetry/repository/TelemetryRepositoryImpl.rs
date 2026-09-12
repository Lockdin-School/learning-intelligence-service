use crate::core::telemetry::domain::TelemetryEvent::TelemetryEvent;
use crate::core::telemetry::domain::TelemetryEventType::TelemetryEventType;
use crate::core::telemetry::repository::TelemetryRepository::TelemetryRepository;
use crate::core::telemetry::dto::TelemetryEventDbDTO::TelemetryEventDbDTO;

pub struct TelemetryRepositoryImpl {
    pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl TelemetryRepository for TelemetryRepositoryImpl {
    async fn save(
        &self,
        event: TelemetryEvent,
    ) -> Result<TelemetryEventDbDTO, sqlx::Error> {
        let event_type = event.event.as_str();
        let payload = serde_json::to_value(&event.event).expect("Failed to serialize event");

        let saved = sqlx::query_as(
            "
        INSERT INTO telemetry_events (
            id,
            student_id,
            session_id,
            event_type,
            payload,
            occurred_at
        )
        VALUES ($1, $2, $3, $4, $5, $6) RETURNING *
        ", )
            .bind(event.id)
            .bind(event.student_id)
            .bind(event.session_id)
            .bind(event_type)
            .bind(payload)
            .bind(event.occurred_at)
            .fetch_one(&self.pool)
            .await?;

        Ok(saved)
    }

    async fn get_all(&self) -> Result<Vec<TelemetryEvent>, sqlx::Error> {
        todo!()
    }

    async fn get_by_type(
        &self,
        event_type: TelemetryEventType,
    ) -> Result<Vec<TelemetryEvent>, sqlx::Error> {
        todo!()
    }
}

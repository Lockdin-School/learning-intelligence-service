use uuid::Uuid;
use crate::core::telemetry::domain::TelemetryEventType::TelemetryEvent;

pub struct EventWrapper {
    pub id: Uuid,
    pub student_id: Uuid,
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub session_id: Uuid,
    pub event: TelemetryEvent,
}
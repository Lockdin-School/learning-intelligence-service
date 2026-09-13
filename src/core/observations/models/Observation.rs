use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::core::observations::models::ObservationType::ObservationType;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Observation {
    pub id: Uuid,

    pub event_type: ObservationType,
    pub version: i32,

    pub student_id: Uuid,

    pub occurred_at: DateTime<Utc>,

    /// Source of the observation.
    pub source_service: String,
    pub source_event_id: Uuid,

    /// Observation context
    pub data: serde_json::Value,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationNew {
    pub event_type: ObservationType,
    pub version: i32,

    pub student_id: Uuid,

    pub occurred_at: DateTime<Utc>,

    pub source_service: String,
    pub source_event_id: Uuid,

    pub data: serde_json::Value,
}
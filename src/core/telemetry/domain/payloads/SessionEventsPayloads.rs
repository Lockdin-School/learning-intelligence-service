use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Payload recorded when a student starts a learning session.
///
/// This event marks the beginning of a student's active learning session.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionStartedPayload {
    pub session_id: Uuid,
}

/// Payload recorded when a student ends a learning session.
///
/// This event marks the end of a student's active learning session.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionEndedPayload {
    pub session_id: Uuid,
}
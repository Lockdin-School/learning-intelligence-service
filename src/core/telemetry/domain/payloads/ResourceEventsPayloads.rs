use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Payload recorded when a student views a learning resource.
///
/// This event represents the student opening or otherwise accessing a
/// resource. It does not imply that the resource was fully consumed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceViewedPayload {
    pub resource_id: Uuid,
}

/// Payload recorded when a student downloads a learning resource.
///
/// This event represents a completed download initiated by the student.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceDownloadedPayload {
    pub resource_id: Uuid,
}
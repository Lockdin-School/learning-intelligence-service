use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Payload recorded when a student views an assignment.
///
/// This event represents the student opening or otherwise accessing an
/// assignment. It does not imply that the student has started working on it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignmentViewedPayload {
    pub assignment_id: Uuid,
}

/// Payload recorded when a student starts an assignment.
///
/// This event represents the beginning of the student's work on an assignment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignmentStartedPayload {
    pub assignment_id: Uuid,
}

/// Payload recorded when a student abandons an assignment.
///
/// This event represents the student leaving an assignment without submitting
/// it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignmentAbandonedPayload {
    pub assignment_id: Uuid,
}

/// Payload recorded when a student submits an assignment.
///
/// This event represents the student's explicit submission of an assignment
/// for evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignmentSubmittedPayload {
    pub assignment_id: Uuid,
}

/// Payload recorded when a student is issued a warning that an assignment is
/// approaching its due date.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignmentDueDateWarningIssuedPayload {
    pub assignment_id: Uuid,
    pub due_at: chrono::DateTime<chrono::Utc>,
}

/// Payload recorded when an assignment becomes overdue.
///
/// This event represents the assignment crossing its configured due date
/// without having been submitted.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignmentOverdueWarningIssuedPayload {
    pub assignment_id: Uuid,
    pub due_at: chrono::DateTime<chrono::Utc>,
}
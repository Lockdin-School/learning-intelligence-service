use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Payload recorded when a student views an exercise.
///
/// This event represents the student opening or otherwise accessing an
/// exercise. It does not imply that the student has started or attempted it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseViewedPayload {
    pub exercise_id: Uuid,
}

/// Payload recorded when a student starts an exercise.
///
/// This event represents the beginning of an exercise attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseStartedPayload {
    pub exercise_id: Uuid,
    pub attempt_id: Uuid,
}

/// Payload recorded when a student attempts to answer an exercise question.
///
/// The payload captures the submitted response and question context. Whether
/// the response is correct, partially correct, or incorrect is determined by
/// downstream processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseQuestionAttemptedPayload {
    pub exercise_id: Uuid,
    pub attempt_id: Uuid,
    pub question_id: Uuid,
    pub response: serde_json::Value,
}

/// Payload recorded when a student abandons an exercise.
///
/// This event represents an exercise attempt being ended without a submission
/// or completion event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseAbandonedPayload {
    pub exercise_id: Uuid,
    pub attempt_id: Uuid,
}

/// Payload recorded when a student completes an exercise.
///
/// This event represents the student reaching the completion state of an
/// exercise. It does not determine the student's performance or mastery.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseCompletedPayload {
    pub exercise_id: Uuid,
    pub attempt_id: Uuid,
}

/// Payload recorded when a student submits an exercise.
///
/// This event represents the student's explicit submission of an exercise
/// attempt for evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseSubmittedPayload {
    pub exercise_id: Uuid,
    pub attempt_id: Uuid,
}

/// Payload recorded when a student is issued a warning that an exercise is
/// approaching its due date.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseDueWarningIssuedPayload {
    pub exercise_id: Uuid,
    pub due_at: chrono::DateTime<chrono::Utc>,
}

/// Payload recorded when an exercise becomes overdue.
///
/// This event represents the exercise crossing its configured due date.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseOverdueWarningIssuedPayload {
    pub exercise_id: Uuid,
    pub due_at: chrono::DateTime<chrono::Utc>,
}
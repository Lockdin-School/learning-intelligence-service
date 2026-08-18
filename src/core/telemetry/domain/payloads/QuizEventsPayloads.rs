use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Payload recorded when a student views a quiz.
///
/// This event represents the student opening or otherwise accessing a quiz.
/// It does not imply that the student has started an attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizViewedPayload {
    pub quiz_id: Uuid,
}

/// Payload recorded when a student starts a quiz.
///
/// This event represents the beginning of a quiz attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizStartedPayload {
    pub quiz_id: Uuid,
    pub attempt_id: Uuid,
}

/// Payload recorded when a student attempts to answer a quiz question.
///
/// The payload captures the submitted response and question context. Whether
/// the response is correct or incorrect is determined by downstream processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizQuestionAttemptedPayload {
    pub quiz_id: Uuid,
    pub attempt_id: Uuid,
    pub question_id: Uuid,
    pub response: serde_json::Value,
}

/// Payload recorded when a student abandons a quiz.
///
/// This event represents a quiz attempt being ended without submission or
/// completion.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizAbandonedPayload {
    pub quiz_id: Uuid,
    pub attempt_id: Uuid,
}

/// Payload recorded when a student completes a quiz.
///
/// This event represents the student reaching the completion state of a quiz.
/// It does not determine the student's performance or mastery.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizCompletedPayload {
    pub quiz_id: Uuid,
    pub attempt_id: Uuid,
}

/// Payload recorded when a student submits a quiz.
///
/// This event represents the student's explicit submission of a quiz attempt
/// for evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizSubmittedPayload {
    pub quiz_id: Uuid,
    pub attempt_id: Uuid,
}

/// Payload recorded when a quiz attempt reaches its configured duration limit.
///
/// This event represents the system ending the attempt because the allotted
/// duration has expired.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizDurationExpiredPayload {
    pub quiz_id: Uuid,
    pub attempt_id: Uuid,
    pub duration_seconds: i32,
}
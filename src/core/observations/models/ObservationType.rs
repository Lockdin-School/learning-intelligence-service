use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
#[sqlx(type_name = "observation_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ObservationType {
    QuizAttemptGraded,
    LessonCompleted,
    VideoWatched,
    QuestionAnswered,
    QuizAttemptStarted,
    AssignmentSubmitted,
    LiveSessionAttended,
}

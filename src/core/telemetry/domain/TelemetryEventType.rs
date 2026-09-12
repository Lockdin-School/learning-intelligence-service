use serde::{Deserialize, Serialize};
use crate::core::telemetry::domain::TelemetryEventError::TelemetryEventError;
use crate::core::telemetry::domain::payloads::AssignmentEventsPayloads::{
    AssignmentAbandonedPayload, AssignmentDueDateWarningIssuedPayload,
    AssignmentOverdueWarningIssuedPayload, AssignmentStartedPayload, AssignmentSubmittedPayload,
    AssignmentViewedPayload,
};
use crate::core::telemetry::domain::payloads::ExerciseEventsPayloads::{
    ExerciseAbandonedPayload, ExerciseCompletedPayload, ExerciseDueWarningIssuedPayload,
    ExerciseOverdueWarningIssuedPayload, ExerciseQuestionAttemptedPayload, ExerciseStartedPayload,
    ExerciseSubmittedPayload, ExerciseViewedPayload,
};
use crate::core::telemetry::domain::payloads::LessonVideoEventPayloads::{
    LessonVideoCompletedPayload, LessonVideoPausedPayload, LessonVideoResumedPayload,
    LessonVideoSeekedPayload, LessonVideoStartedPayload, LessonVideoViewedPayload,
};
use crate::core::telemetry::domain::payloads::QuizEventsPayloads::{
    QuizAbandonedPayload, QuizCompletedPayload, QuizDurationExpiredPayload,
    QuizQuestionAttemptedPayload, QuizStartedPayload, QuizSubmittedPayload, QuizViewedPayload,
};
use crate::core::telemetry::domain::payloads::ResourceEventsPayloads::{
    ResourceDownloadedPayload, ResourceViewedPayload,
};
use crate::core::telemetry::domain::payloads::SessionEventsPayloads::{
    SessionEndedPayload, SessionStartedPayload,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TelemetryEventType {
    // Session
    SessionStarted(SessionStartedPayload),
    SessionEnded(SessionEndedPayload),
    // Lessons
    LessonVideoViewed(LessonVideoViewedPayload),
    LessonVideoStarted(LessonVideoStartedPayload),
    LessonVideoPaused(LessonVideoPausedPayload),
    LessonVideoResumed(LessonVideoResumedPayload),
    LessonVideoSeeked(LessonVideoSeekedPayload),
    LessonVideoCompleted(LessonVideoCompletedPayload),
    // Exercises
    ExerciseViewed(ExerciseViewedPayload),
    ExerciseStarted(ExerciseStartedPayload),
    ExerciseQuestionAttempted(ExerciseQuestionAttemptedPayload),
    ExerciseAbandoned(ExerciseAbandonedPayload),
    ExerciseCompleted(ExerciseCompletedPayload),
    ExerciseSubmitted(ExerciseSubmittedPayload),
    ExerciseDueWarningIssued(ExerciseDueWarningIssuedPayload),
    ExerciseOverdueWarningIssued(ExerciseOverdueWarningIssuedPayload),
    // Quizzes
    QuizViewed(QuizViewedPayload),
    QuizStarted(QuizStartedPayload),
    QuizQuestionAttempted(QuizQuestionAttemptedPayload),
    QuizAbandoned(QuizAbandonedPayload),
    QuizCompleted(QuizCompletedPayload),
    QuizSubmitted(QuizSubmittedPayload),
    QuizDurationExpired(QuizDurationExpiredPayload),
    // Assignments
    AssignmentViewed(AssignmentViewedPayload),
    AssignmentStarted(AssignmentStartedPayload),
    AssignmentAbandoned(AssignmentAbandonedPayload),
    AssignmentSubmitted(AssignmentSubmittedPayload),
    AssignmentDueDateWarningIssued(AssignmentDueDateWarningIssuedPayload),
    AssignmentOverdueWarningIssued(AssignmentOverdueWarningIssuedPayload),
    // Resources
    ResourceViewed(ResourceViewedPayload),
    ResourceDownloaded(ResourceDownloadedPayload),
}

impl TelemetryEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            // Session
            Self::SessionStarted(_) => "session.started",
            Self::SessionEnded(_) => "session.ended",

            // Lessons
            Self::LessonVideoViewed(_) => "lesson_video.viewed",
            Self::LessonVideoStarted(_) => "lesson_video.started",
            Self::LessonVideoPaused(_) => "lesson_video.paused",
            Self::LessonVideoResumed(_) => "lesson_video.resumed",
            Self::LessonVideoSeeked(_) => "lesson_video.seeked",
            Self::LessonVideoCompleted(_) => "lesson_video.completed",

            // Exercises
            Self::ExerciseViewed(_) => "exercise.viewed",
            Self::ExerciseStarted(_) => "exercise.started",
            Self::ExerciseQuestionAttempted(_) => "exercise.question_attempted",
            Self::ExerciseAbandoned(_) => "exercise.abandoned",
            Self::ExerciseCompleted(_) => "exercise.completed",
            Self::ExerciseSubmitted(_) => "exercise.submitted",
            Self::ExerciseDueWarningIssued(_) => "exercise.due_warning_issued",
            Self::ExerciseOverdueWarningIssued(_) => "exercise.overdue_warning_issued",

            // Quizzes
            Self::QuizViewed(_) => "quiz.viewed",
            Self::QuizStarted(_) => "quiz.started",
            Self::QuizQuestionAttempted(_) => "quiz.question_attempted",
            Self::QuizAbandoned(_) => "quiz.abandoned",
            Self::QuizCompleted(_) => "quiz.completed",
            Self::QuizSubmitted(_) => "quiz.submitted",
            Self::QuizDurationExpired(_) => "quiz.duration_expired",

            // Assignments
            Self::AssignmentViewed(_) => "assignment.viewed",
            Self::AssignmentStarted(_) => "assignment.started",
            Self::AssignmentAbandoned(_) => "assignment.abandoned",
            Self::AssignmentSubmitted(_) => "assignment.submitted",
            Self::AssignmentDueDateWarningIssued(_) => "assignment.due_date_warning_issued",
            Self::AssignmentOverdueWarningIssued(_) => "assignment.overdue_warning_issued",

            // Resources
            Self::ResourceViewed(_) => "resource.viewed",
            Self::ResourceDownloaded(_) => "resource.downloaded",
        }
    }

    pub fn from_parts(
        event_type: &str,
        payload: serde_json::Value,
    ) -> Result<Self, TelemetryEventError> {
        macro_rules! deserialize {
            ($payload:expr, $event_type:expr, $ty:ty, $variant:path) => {
                Ok($variant(serde_json::from_value::<$ty>($payload).map_err(
                    |source| TelemetryEventError::InvalidPayload {
                        event_type: $event_type.to_owned(),
                        source,
                    },
                )?))
            };
        }

        match event_type {
            // Session
            "session.started" => deserialize!(
                payload,
                event_type,
                SessionStartedPayload,
                Self::SessionStarted
            ),
            "session.ended" => {
                deserialize!(payload, event_type, SessionEndedPayload, Self::SessionEnded)
            }

            // Lessons
            "lesson_video.viewed" => deserialize!(
                payload,
                event_type,
                LessonVideoViewedPayload,
                Self::LessonVideoViewed
            ),
            "lesson_video.started" => deserialize!(
                payload,
                event_type,
                LessonVideoStartedPayload,
                Self::LessonVideoStarted
            ),
            "lesson_video.paused" => deserialize!(
                payload,
                event_type,
                LessonVideoPausedPayload,
                Self::LessonVideoPaused
            ),
            "lesson_video.resumed" => deserialize!(
                payload,
                event_type,
                LessonVideoResumedPayload,
                Self::LessonVideoResumed
            ),
            "lesson_video.seeked" => deserialize!(
                payload,
                event_type,
                LessonVideoSeekedPayload,
                Self::LessonVideoSeeked
            ),
            "lesson_video.completed" => deserialize!(
                payload,
                event_type,
                LessonVideoCompletedPayload,
                Self::LessonVideoCompleted
            ),

            // Exercises
            "exercise.viewed" => deserialize!(
                payload,
                event_type,
                ExerciseViewedPayload,
                Self::ExerciseViewed
            ),
            "exercise.started" => deserialize!(
                payload,
                event_type,
                ExerciseStartedPayload,
                Self::ExerciseStarted
            ),
            "exercise.question_attempted" => deserialize!(
                payload,
                event_type,
                ExerciseQuestionAttemptedPayload,
                Self::ExerciseQuestionAttempted
            ),
            "exercise.abandoned" => deserialize!(
                payload,
                event_type,
                ExerciseAbandonedPayload,
                Self::ExerciseAbandoned
            ),
            "exercise.completed" => deserialize!(
                payload,
                event_type,
                ExerciseCompletedPayload,
                Self::ExerciseCompleted
            ),
            "exercise.submitted" => deserialize!(
                payload,
                event_type,
                ExerciseSubmittedPayload,
                Self::ExerciseSubmitted
            ),
            "exercise.due_warning_issued" => deserialize!(
                payload,
                event_type,
                ExerciseDueWarningIssuedPayload,
                Self::ExerciseDueWarningIssued
            ),
            "exercise.overdue_warning_issued" => deserialize!(
                payload,
                event_type,
                ExerciseOverdueWarningIssuedPayload,
                Self::ExerciseOverdueWarningIssued
            ),

            // Quizzes
            "quiz.viewed" => deserialize!(payload, event_type, QuizViewedPayload, Self::QuizViewed),
            "quiz.started" => {
                deserialize!(payload, event_type, QuizStartedPayload, Self::QuizStarted)
            }
            "quiz.question_attempted" => deserialize!(
                payload,
                event_type,
                QuizQuestionAttemptedPayload,
                Self::QuizQuestionAttempted
            ),
            "quiz.abandoned" => deserialize!(
                payload,
                event_type,
                QuizAbandonedPayload,
                Self::QuizAbandoned
            ),
            "quiz.completed" => deserialize!(
                payload,
                event_type,
                QuizCompletedPayload,
                Self::QuizCompleted
            ),
            "quiz.submitted" => deserialize!(
                payload,
                event_type,
                QuizSubmittedPayload,
                Self::QuizSubmitted
            ),
            "quiz.duration_expired" => deserialize!(
                payload,
                event_type,
                QuizDurationExpiredPayload,
                Self::QuizDurationExpired
            ),

            // Assignments
            "assignment.viewed" => deserialize!(
                payload,
                event_type,
                AssignmentViewedPayload,
                Self::AssignmentViewed
            ),
            "assignment.started" => deserialize!(
                payload,
                event_type,
                AssignmentStartedPayload,
                Self::AssignmentStarted
            ),
            "assignment.abandoned" => deserialize!(
                payload,
                event_type,
                AssignmentAbandonedPayload,
                Self::AssignmentAbandoned
            ),
            "assignment.submitted" => deserialize!(
                payload,
                event_type,
                AssignmentSubmittedPayload,
                Self::AssignmentSubmitted
            ),
            "assignment.due_date_warning_issued" => deserialize!(
                payload,
                event_type,
                AssignmentDueDateWarningIssuedPayload,
                Self::AssignmentDueDateWarningIssued
            ),
            "assignment.overdue_warning_issued" => deserialize!(
                payload,
                event_type,
                AssignmentOverdueWarningIssuedPayload,
                Self::AssignmentOverdueWarningIssued
            ),

            // Resources
            "resource.viewed" => deserialize!(
                payload,
                event_type,
                ResourceViewedPayload,
                Self::ResourceViewed
            ),
            "resource.downloaded" => deserialize!(
                payload,
                event_type,
                ResourceDownloadedPayload,
                Self::ResourceDownloaded
            ),

            _ => Err(TelemetryEventError::UnknownEventType(event_type.to_owned())),
        }
    }
}

impl std::fmt::Display for TelemetryEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

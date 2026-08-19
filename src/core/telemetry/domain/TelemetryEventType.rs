use crate::core::telemetry::domain::payloads::AssignmentEventsPayloads::{AssignmentAbandonedPayload, AssignmentDueDateWarningIssuedPayload, AssignmentStartedPayload, AssignmentSubmittedPayload, AssignmentViewedPayload};
use crate::core::telemetry::domain::payloads::ExerciseEventsPayloads::{
    ExerciseAbandonedPayload, ExerciseCompletedPayload, ExerciseDueWarningIssuedPayload,
    ExerciseQuestionAttemptedPayload, ExerciseStartedPayload, ExerciseSubmittedPayload,
    ExerciseViewedPayload,
};
use crate::core::telemetry::domain::payloads::LessonVideoEventPayloads::{
    LessonVideoCompletedPayload, LessonVideoPausedPayload, LessonVideoResumedPayload,
    LessonVideoSeekedPayload, LessonVideoStartedPayload, LessonVideoViewedPayload,
};
use crate::core::telemetry::domain::payloads::QuizEventsPayloads::{QuizAbandonedPayload, QuizCompletedPayload, QuizDurationExpiredPayload, QuizQuestionAttemptedPayload, QuizStartedPayload, QuizSubmittedPayload, QuizViewedPayload};
use crate::core::telemetry::domain::payloads::ResourceEventsPayloads::{ResourceDownloadedPayload, ResourceViewedPayload};
use crate::core::telemetry::domain::payloads::SessionEventsPayloads::{SessionEndedPayload, SessionStartedPayload};

pub enum TelemetryEvent {
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
    ExerciseOverdueWarningIssued(ExerciseDueWarningIssuedPayload),
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
    AssignmentOverdueWarningIssued(AssignmentDueDateWarningIssuedPayload),
    // Resources
    ResourceViewed(ResourceViewedPayload),
    ResourceDownloaded(ResourceDownloadedPayload),
}

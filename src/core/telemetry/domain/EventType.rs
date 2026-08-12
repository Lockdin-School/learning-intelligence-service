use crate::core::telemetry::domain::payloads::LessonCompleted::LessonCompletedPayload;
use crate::core::telemetry::domain::payloads::LessonStarted::LessonStartedPayload;
use crate::core::telemetry::domain::payloads::QuestionAttempted::QuestionAttemptedPayload;
use crate::core::telemetry::domain::payloads::VideoPaused::VideoPausedPayload;
use crate::core::telemetry::domain::payloads::VideoResumed::VideoResumedPayload;
use crate::core::telemetry::domain::payloads::VideoSeeked::VideoSeekedPayload;
use crate::core::telemetry::domain::payloads::VideoStarted::VideoStartedPayload;

pub enum TelemetryEvent {
    VideoStarted(VideoStartedPayload),
    VideoPaused(VideoPausedPayload),
    VideoResumed(VideoResumedPayload),
    VideoSeeked(VideoSeekedPayload),
    LessonStarted(LessonStartedPayload),
    LessonCompleted(LessonCompletedPayload),
    QuestionAttempted(QuestionAttemptedPayload),
    // more event types
}
//! Telemetry payloads for lesson video playback events.
//!
//! These types define the strongly typed application-level representation of
//! video-related telemetry events. They are serialized to JSON for persistence
//! in the telemetry event store.
//!
//! The payloads intentionally contain only factual information about the
//! student's interaction with a lesson video. Interpretation and derived
//! learning signals are handled by the transformation layer.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Payload recorded when a student views a lesson video.
///
/// This event represents the student opening or otherwise viewing a video.
/// It does not imply that playback has started or that any meaningful portion
/// of the video has been watched.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonVideoViewedPayload {
    pub lesson_id: Uuid,
    pub video_id: Uuid,
}

/// Payload recorded when a student starts or begins playing a lesson video.
///
/// `position_seconds` represents the playback position at which the video
/// started.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonVideoStartedPayload {
    pub lesson_id: Uuid,
    pub video_id: Uuid,
    pub position_seconds: i32,
}

/// Payload recorded when a student pauses a lesson video.
///
/// `position_seconds` represents the playback position at which the pause
/// occurred.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonVideoPausedPayload {
    pub lesson_id: Uuid,
    pub video_id: Uuid,
    pub position_seconds: i32,
}

/// Payload recorded when a student resumes a previously paused lesson video.
///
/// `position_seconds` represents the playback position at which playback
/// resumed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonVideoResumedPayload {
    pub lesson_id: Uuid,
    pub video_id: Uuid,
    pub position_seconds: i32,
}

/// Payload recorded when a student seeks to a different position in a lesson
/// video.
///
/// Both the position before the seek and the destination position are
/// preserved so that downstream processing can determine the direction and
/// magnitude of the seek.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonVideoSeekedPayload {
    pub lesson_id: Uuid,
    pub video_id: Uuid,
    pub from_position_seconds: i32,
    pub to_position_seconds: i32,
}

/// Payload recorded when a student completes a lesson video.
///
/// `position_seconds` represents the playback position at which completion
/// was recorded.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonVideoCompletedPayload {
    pub lesson_id: Uuid,
    pub video_id: Uuid,
    pub position_seconds: i32,
}
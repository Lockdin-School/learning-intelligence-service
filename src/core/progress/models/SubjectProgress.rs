use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use crate::core::progress::models::TopicProgress::TopicProgress;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectProgress {
    pub subject_id: Uuid,
    pub mastery: f64,
    pub questions_answered: i32,
    pub last_attempted_at: DateTime<Utc>,
    pub topics: Vec<TopicProgress>,
}
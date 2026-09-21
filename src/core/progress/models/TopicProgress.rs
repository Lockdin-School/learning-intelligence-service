use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use crate::core::progress::service::ProgressService::ConceptProgress;

#[derive(Debug, Clone, Serialize)]
pub struct TopicProgress {
    pub topic_id: Uuid,
    pub mastery: f64,
    pub questions_answered: i32,
    pub last_attempted_at: DateTime<Utc>,
    pub concepts: Vec<ConceptProgress>,
}


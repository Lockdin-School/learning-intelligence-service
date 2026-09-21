use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConceptProgressSignal {
    pub student_id: Uuid,
    pub concept_id: Uuid,
    pub topic_id: Uuid,
    pub subject_id: Uuid,
    pub correct: bool,
    pub source_observation_id: Uuid,
    pub occurred_at: DateTime<Utc>,
}

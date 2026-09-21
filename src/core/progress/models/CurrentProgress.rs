use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct CurrentProgress {
    pub student_id: Uuid,
    pub concept_id: Uuid,
    pub questions_answered: i32,
    pub questions_correct: i32,
    pub accuracy: Decimal, // Postgres-generated — never set this from Rust
    pub last_attempted_at: DateTime<Utc>,
}

impl From<CurrentProgressRow> for CurrentProgress {
    fn from(value: CurrentProgressRow) -> Self {
        CurrentProgress {
            student_id: value.student_id,
            concept_id: value.concept_id,
            questions_answered: value.questions_answered,
            questions_correct: value.questions_correct,
            accuracy: value.accuracy,
            last_attempted_at: value.updated_at,
        }
    }
}

/// Full row as read from `current_progress`.
#[derive(Debug, Clone, FromRow)]
pub struct CurrentProgressRow {
    pub student_id: Uuid,
    pub concept_id: Uuid,
    pub questions_answered: i32,
    pub questions_correct: i32,
    pub accuracy: Decimal, // Postgres-generated — never set this from Rust
    pub updated_at: DateTime<Utc>,
}

/// What ProgressService needs to provide to upsert a `current_progress` row.
/// No `accuracy`, no `updated_at` — Postgres derives/defaults both.
#[derive(Debug, Clone)]
pub struct UpsertCurrentProgress {
    pub student_id: Uuid,
    pub concept_id: Uuid,
    pub questions_answered: i32,
    pub questions_correct: i32,
}


#[derive(Debug, FromRow)]
pub struct ConceptProgressJoinedRow {
    pub subject_id: Uuid,
    pub topic_id: Uuid,
    pub concept_id: Uuid,
    pub questions_answered: i32,
    pub questions_correct: i32,
    pub accuracy: Decimal,
    pub last_attempted_at: DateTime<Utc>,
}
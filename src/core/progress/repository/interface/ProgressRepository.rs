use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Postgres, Transaction};
use thiserror::Error;
use uuid::Uuid;

use crate::core::progress::models::CurrentProgress::{ConceptProgressJoinedRow, CurrentProgressRow};
use crate::core::progress::models::ProgressHistory::{NewProgressHistory, ProgressHistoryRow};

#[derive(Debug, Error)]
pub enum ProgressRepositoryError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

#[derive(Debug, Clone)]
pub struct ConceptHierarchyRef {
    pub concept_id: Uuid,
    pub topic_id: Uuid,
    pub subject_id: Uuid,
}

#[async_trait]
pub trait ProgressRepository: Send + Sync {
    /// Appends a progress_history row and upserts current_progress to match,
    /// within the transaction the caller already holds open. Returns the
    /// inserted history row (id, accuracy, created_at all DB-generated).
    async fn record_progress(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        entry: NewProgressHistory,
        hierarchy: ConceptHierarchyRef
    ) -> Result<ProgressHistoryRow, ProgressRepositoryError>;

    async fn get_current_progress_with_hierarchy(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<ConceptProgressJoinedRow>, ProgressRepositoryError>;

    /// Current standing across every concept the student has any signal for.
    async fn get_current_progress(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<CurrentProgressRow>, ProgressRepositoryError>;

    /// Current standing for one concept, if the student has any signal for it.
    async fn get_current_progress_for_concept(
        &self,
        student_id: Uuid,
        concept_id: Uuid,
    ) -> Result<Option<CurrentProgressRow>, ProgressRepositoryError>;

    /// Full history for one student+concept, most recent first.
    async fn get_history_for_concept(
        &self,
        student_id: Uuid,
        concept_id: Uuid,
    ) -> Result<Vec<ProgressHistoryRow>, ProgressRepositoryError>;

    /// The latest history row at or before `as_of`, per concept — the "past"
    /// side of a trend comparison (e.g. state as of 30 days ago).
    async fn get_progress_as_of(
        &self,
        student_id: Uuid,
        as_of: DateTime<Utc>,
    ) -> Result<Vec<ProgressHistoryRow>, ProgressRepositoryError>;

    /// Rebuilds current_progress entirely from progress_history. Returns
    /// rows affected. The cache-recovery path if the two ever drift, or
    /// after the accuracy formula changes.
    async fn rebuild_current_progress(&self) -> Result<u64, ProgressRepositoryError>;
}

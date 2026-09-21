use crate::core::progress::models::CurrentProgress::{ConceptProgressJoinedRow, CurrentProgressRow};
use crate::core::progress::models::ProgressHistory::{NewProgressHistory, ProgressHistoryRow};
use crate::core::progress::repository::interface::ProgressRepository::{ConceptHierarchyRef, ProgressRepository, ProgressRepositoryError};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;



pub struct PgProgressRepository {
    pool: PgPool,
}

impl PgProgressRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProgressRepository for PgProgressRepository {
    async fn record_progress(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        entry: NewProgressHistory,
        hierarchy: ConceptHierarchyRef
    ) -> Result<ProgressHistoryRow, ProgressRepositoryError> {

        let history_row = sqlx::query_as::<_, ProgressHistoryRow>(
            r#"
            INSERT INTO progress_history
                (student_id, concept_id, observation_id, questions_answered, questions_correct)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, student_id, concept_id, observation_id,
                      questions_answered, questions_correct, accuracy, created_at
            "#,
        )
        .bind(entry.student_id)
        .bind(entry.concept_id)
        .bind(entry.observation_id)
        .bind(entry.questions_answered)
        .bind(entry.questions_correct)
        .fetch_one(tx.as_mut())
        .await?;

        sqlx::query(
            r#"
            INSERT INTO current_progress
                (student_id, concept_id, questions_answered, questions_correct)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (student_id, concept_id) DO UPDATE
            SET questions_answered = EXCLUDED.questions_answered,
                questions_correct  = EXCLUDED.questions_correct,
                updated_at         = now()
            "#,
        )
        .bind(entry.student_id)
        .bind(entry.concept_id)
        .bind(entry.questions_answered)
        .bind(entry.questions_correct)
        .execute(tx.as_mut())
        .await?;

        // Write to concepts_lookup table
        sqlx::query(
            r#"
                INSERT INTO concepts_lookup (concept_id, topic_id, subject_id)
                VALUES ($1, $2, $3)
                ON CONFLICT (concept_id) DO UPDATE
                SET topic_id = EXCLUDED.topic_id, subject_id = EXCLUDED.subject_id
                "#,
        )
            .bind(hierarchy.concept_id)
            .bind(hierarchy.topic_id)
            .bind(hierarchy.subject_id)
            .execute(tx.as_mut())
            .await?;

        Ok(history_row)
    }


    async fn get_current_progress_with_hierarchy(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<ConceptProgressJoinedRow>, ProgressRepositoryError> {
        let rows = sqlx::query_as::<_, ConceptProgressJoinedRow>(
            r#"
        SELECT
            cl.subject_id, cl.topic_id, cp.concept_id,
            cp.questions_answered, cp.questions_correct, cp.accuracy,
            cp.updated_at AS last_attempted_at
        FROM current_progress cp
        JOIN concepts_lookup cl ON cl.concept_id = cp.concept_id
        WHERE cp.student_id = $1
        ORDER BY cl.subject_id, cl.topic_id, cp.concept_id
        "#,
        )
            .bind(student_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }

    async fn get_current_progress(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<CurrentProgressRow>, ProgressRepositoryError> {
        let rows = sqlx::query_as::<_, CurrentProgressRow>(
            r#"
            SELECT student_id, concept_id, questions_answered, questions_correct,
                   accuracy, updated_at
            FROM current_progress
            WHERE student_id = $1
            ORDER BY concept_id
            "#,
        )
        .bind(student_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn get_current_progress_for_concept(
        &self,
        student_id: Uuid,
        concept_id: Uuid,
    ) -> Result<Option<CurrentProgressRow>, ProgressRepositoryError> {
        let row = sqlx::query_as::<_, CurrentProgressRow>(
            r#"
            SELECT student_id, concept_id, questions_answered, questions_correct,
                   accuracy, updated_at
            FROM current_progress
            WHERE student_id = $1 AND concept_id = $2
            "#,
        )
        .bind(student_id)
        .bind(concept_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    async fn get_history_for_concept(
        &self,
        student_id: Uuid,
        concept_id: Uuid,
    ) -> Result<Vec<ProgressHistoryRow>, ProgressRepositoryError> {
        let rows = sqlx::query_as::<_, ProgressHistoryRow>(
            r#"
            SELECT id, student_id, concept_id, observation_id,
                   questions_answered, questions_correct, accuracy, created_at
            FROM progress_history
            WHERE student_id = $1 AND concept_id = $2
            ORDER BY created_at DESC
            "#,
        )
        .bind(student_id)
        .bind(concept_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn get_progress_as_of(
        &self,
        student_id: Uuid,
        as_of: DateTime<Utc>,
    ) -> Result<Vec<ProgressHistoryRow>, ProgressRepositoryError> {
        let rows = sqlx::query_as::<_, ProgressHistoryRow>(
            r#"
            SELECT DISTINCT ON (concept_id)
                   id, student_id, concept_id, observation_id,
                   questions_answered, questions_correct, accuracy, created_at
            FROM progress_history
            WHERE student_id = $1 AND created_at <= $2
            ORDER BY concept_id, created_at DESC
            "#,
        )
        .bind(student_id)
        .bind(as_of)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn rebuild_current_progress(&self) -> Result<u64, ProgressRepositoryError> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("TRUNCATE current_progress")
            .execute(&mut *tx)
            .await?;

        let result = sqlx::query(
            r#"
            INSERT INTO current_progress
                (student_id, concept_id, questions_answered, questions_correct)
            SELECT DISTINCT ON (student_id, concept_id)
                   student_id, concept_id, questions_answered, questions_correct
            FROM progress_history
            ORDER BY student_id, concept_id, created_at DESC
            "#,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(result.rows_affected())
    }
}

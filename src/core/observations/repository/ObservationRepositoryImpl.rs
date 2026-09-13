use crate::core::observations::models::Observation::{Observation, ObservationNew};
use crate::core::observations::repository::ObservationRepository::ObservationRepository;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgObservationRepository {
    pool: PgPool,
}

impl PgObservationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ObservationRepository for PgObservationRepository {
    async fn create(&self, observation: &ObservationNew) -> sqlx::Result<Observation, sqlx::Error> {
        let id = Uuid::now_v7();

        let observation = sqlx::query_as::<_, Observation>(
            r#"
        INSERT INTO observations (
            id,
            event_type,
            version,
            student_id,
            occurred_at,
            source_service,
            source_event_id,
            data
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING
            id,
            event_type,
            version,
            student_id,
            occurred_at,
            source_service,
            source_event_id,
            data,
            created_at
        "#,
        )
        .bind(id)
        .bind(observation.event_type)
        .bind(observation.version)
        .bind(observation.student_id)
        .bind(observation.occurred_at)
        .bind(&observation.source_service)
        .bind(observation.source_event_id)
        .bind(&observation.data)
        .fetch_one(&self.pool)
        .await?;

        Ok(observation)
    }

    async fn find_by_id(&self, id: Uuid) -> sqlx::Result<Option<Observation>> {
        sqlx::query_as(
            "
            SELECT
                id,
                event_type,
                version,
                student_id,
                occurred_at,
                source_service,
                source_event_id,
                data,
                created_at
            FROM observations
            WHERE id = $1
            ",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn exists_by_source_event(
        &self,
        source_service: &str,
        source_event_id: Uuid,
    ) -> sqlx::Result<bool> {
        let exists = sqlx::query_scalar::<_, bool>(
            r#"
        SELECT EXISTS (
            SELECT 1
            FROM observations
            WHERE source_service = $1
              AND source_event_id = $2
        )
        "#,
        )
        .bind(source_service)
        .bind(source_event_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(exists)
    }
}

use std::sync::Arc;
use crate::core::observation::service::ObservationService::ObservationService;
use crate::infrastructure::db::database::{init_postgres, run_migrations};
use actix_web::web::Data;
use sqlx::PgPool;
use crate::core::observation::repository::ObservationRepositoryImpl::PgObservationRepository;

#[derive(Clone)]
pub struct AppState {
    pub observation_service: Data<ObservationService>,
}

pub fn app_state(pg_pool: PgPool) -> AppState {
    AppState {
        observation_service: Data::new(ObservationService::new(
            Arc::new(PgObservationRepository::new(pg_pool.clone())),
        )),
    }
}

pub async fn init_state() -> AppState {
    log::info!("Initializing state...");
    let pg_pool = init_postgres().await;
    // let redis = init_redis().await.expect("Failed to initialize redis");

    run_migrations(&pg_pool).await;
    app_state(pg_pool)
}

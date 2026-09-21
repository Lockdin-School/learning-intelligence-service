use crate::configuration::events::event_handlers_init;
use crate::core::observations::repository::ObservationRepositoryImpl::PgObservationRepository;
use crate::core::observations::service::ObservationService::ObservationService;
use crate::core::progress::repository::implementations::ProgressRepositoryImpl::PgProgressRepository;
use crate::core::progress::service::ProgressService::ProgressService;
use crate::core::transformation::transformers::QAGTransformer::QAGTransformer;
use crate::infrastructure::InternalEventBus::{EventBus, init_event_bus};
use crate::infrastructure::db::database::{init_postgres, run_migrations};
use actix_web::web::Data;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub observation_service: Data<ObservationService>,
    pub progress_service: Data<ProgressService>,
    pub qag_transformer: Data<QAGTransformer>,
}

pub fn app_state(pg_pool: PgPool, event_bus: Data<EventBus>) -> AppState {
    AppState {
        // Services
        observation_service: Data::new(ObservationService::new(Arc::new(
            PgObservationRepository::new(pg_pool.clone()), ),
            event_bus.clone()
        )),
        progress_service: Data::new(ProgressService::new(
            Arc::new(PgProgressRepository::new(pg_pool.clone())),
            pg_pool.clone(),
        )),

        // Transformers
        qag_transformer: Data::new(QAGTransformer::new(event_bus.clone())),
    }
}

pub async fn init_state() -> AppState {
    log::info!("state.initialise | configuration | init_state | Started | Initialising state... ");
    let pg_pool = init_postgres().await;
    let tx = init_event_bus();
    let event_bus = Data::new(tx.clone());

    run_migrations(&pg_pool).await;
    let state = app_state(pg_pool, event_bus);

    // --- EVENT SUBSCRIBERS ---
    event_handlers_init(tx, Data::new(state.clone())).await;

    state
}

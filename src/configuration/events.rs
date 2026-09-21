use crate::configuration::state::AppState;
use crate::infrastructure::InternalEventBus::Event;
use actix_web::web::Data;
use std::sync::Arc;
use tokio::sync::broadcast::Sender;

pub async fn event_handlers_init(tx: Sender<Event>, state: Data<AppState>) {
    // --- SPAWN QAGTransformer SUBSCRIBER ---
    let rx_qag_transformer = tx.subscribe();
    let qag_transformer = Arc::clone(&Arc::new(state.qag_transformer.clone()));
    tokio::spawn(async move {
        qag_transformer.events_handler(rx_qag_transformer).await;
    });

    // --- SPAWN Progress SUBSCRIBER ---
    let rx_progress = tx.subscribe();
    let progress_service = Arc::clone(&Arc::new(state.progress_service.clone()));
    tokio::spawn(async move {
        progress_service.events_handler(rx_progress).await;
    });
}

use crate::core::observations::models::Observation::Observation;
use crate::core::progress::models::ProgressSignal::ConceptProgressSignal;
use tokio::sync::broadcast;

pub type EventBus = broadcast::Sender<Event>;

#[derive(Debug, Clone)]
pub enum Event {
    QuizAttemptGraded(Observation),
    ConceptProgressSignalsGenerated(Vec<ConceptProgressSignal>),
}

pub fn init_event_bus() -> broadcast::Sender<Event> {
    let (tx, _rx) = broadcast::channel(16);
    tx
}

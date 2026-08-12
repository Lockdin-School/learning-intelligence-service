use tokio::sync::broadcast;

pub type EventBus = broadcast::Sender<Event>;

#[derive(Debug, Clone)]
pub enum Event {
    // list all events
}

pub fn init_event_bus() -> broadcast::Sender<Event> {
    let (tx, _rx) = broadcast::channel(16);
    tx
}

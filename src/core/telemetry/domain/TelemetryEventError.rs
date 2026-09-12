use thiserror::Error;

#[derive(Debug, Error)]
pub enum TelemetryEventError {
    #[error("unknown telemetry event type: {0}")]
    UnknownEventType(String),

    #[error("invalid payload for telemetry event type '{event_type}': {source}")]
    InvalidPayload {
        event_type: String,
        #[source]
        source: serde_json::Error,
    },
}
use nats::asynk::Message as NatsMessage;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct NatsEvent {
    /// See: https://docs.rs/nats/latest/nats/asynk/struct.Message.html
    pub nats_subject: String,
    pub nats_reply: Option<String>,
    pub nats_data: Vec<u8>,
}

#[derive(Error, Debug)]
pub enum NatsEventError {
    #[error("Internal failure to convert Nats Event to JSON String: {0}")]
    InternalConversion(String),
}

impl TryFrom<NatsEvent> for String {
    type Error = NatsEventError;
    fn try_from(event: NatsEvent) -> Result<Self, NatsEventError> {
        serde_json::to_string(&event)
            .map_err(|e| NatsEventError::InternalConversion(e.to_string()))
    }
}

impl From<NatsMessage> for NatsEvent {
    fn from(msg: NatsMessage) -> Self {
        Self {
            nats_subject: msg.subject,
            nats_reply: msg.reply,
            nats_data: msg.data,
        }
    }
}

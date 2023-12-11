use nats::asynk::Message as NatsMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct NatsEvent {
    /// See: https://docs.rs/nats/latest/nats/asynk/struct.Message.html
    pub nats_subject: String,
    pub nats_reply: Option<String>,
    pub nats_data: Vec<u8>,
}

impl From<NatsEvent> for String {
    fn from(event: NatsEvent) -> Self {
        serde_json::to_string(&event).unwrap()
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

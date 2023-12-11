use anyhow::Error as AnyhowError;

use fluvio::FluvioError;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NatsConnectorError {
    #[error("Fluvio error: `{0}`.")]
    Fluvio(#[from] FluvioError),
    #[error("Serde Json Error: `{0}`.")]
    SerdeJson(#[from] SerdeJsonError),
    #[error("Anyhow Error: `{0:#?}`.")]
    Anyhow(#[from] AnyhowError),
}

//impl From<NatsError> for NatsConnectorError {
//    fn from(e: NatsError) -> Self {
//        Self::Nats(e)
//    }
//}

use fluvio_connector_common::connector;

#[connector(config, name = "nats")]
#[derive(Debug)]
pub(crate) struct NatsConfig {
    pub host: String,
    pub subject: String,
}

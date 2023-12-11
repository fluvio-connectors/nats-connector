use crate::{config::NatsConfig, event::NatsEvent};
use anyhow::Result;
use async_std::channel::{self, Sender};
use async_std::task::spawn;
use async_trait::async_trait;
use fluvio::Offset;
//use fluvio::{dataplane::record::{Record, RecordData}, Offset, RecordKey};
use fluvio_connector_common::tracing::info;
use fluvio_connector_common::Source;
use futures::{stream::LocalBoxStream, StreamExt};
use nats::asynk::Subscription as NatsSubscription;

const CHANNEL_BUFFER_SIZE: usize = 10000;

pub(crate) struct NatsSource {
    host: String,
    subject: String,
}

impl NatsSource {
    pub(crate) fn new(config: &NatsConfig) -> Result<Self> {
        let host = config.host.clone();
        let subject = config.subject.clone();
        Ok(Self { host, subject })
    }
}

#[async_trait]
impl<'a> Source<'a, String> for NatsSource {
    async fn connect(self, _offset: Option<Offset>) -> Result<LocalBoxStream<'a, String>> {
        info!("Nats host: {} subject {}", &self.host, &self.subject);

        let nats_client = nats::asynk::connect(self.host).await?;
        let nats_subscription = nats_client.subscribe(&self.subject).await?;

        info!(
            "Nats client_id {} ip {}",
            nats_client.client_id(),
            nats_client.client_ip().unwrap()
        );

        let (sender, receiver) = channel::bounded(CHANNEL_BUFFER_SIZE);
        spawn(nats_loop(sender, nats_client, nats_subscription));
        Ok(receiver.boxed_local())
    }
}

use nats::asynk::Connection as NatsConnection;

async fn nats_loop(
    tx: Sender<String>,
    _nats_client: NatsConnection,
    nats_sub: NatsSubscription,
) -> Result<()> {
    info!("Nats loop started");

    loop {
        while let Some(msg) = nats_sub.next().await {
            info!("Nats got: {:?}", msg);
            let nats_event: NatsEvent = msg.into();
            tx.send(nats_event.into()).await?;
        }
    }
}

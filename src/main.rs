mod config;
mod error;
mod event;
mod source;

use config::NatsConfig;

use fluvio::{RecordKey, TopicProducer};
use fluvio_connector_common::{
    connector,
    tracing::{debug, trace},
    Result, Source,
};
use futures::StreamExt;
use source::NatsSource;

#[connector(source)]
async fn start(config: NatsConfig, producer: TopicProducer) -> Result<()> {
    debug!(?config);
    let source = NatsSource::new(&config)?;
    let mut stream = source.connect(None).await?;
    while let Some(item) = stream.next().await {
        trace!(?item);
        producer.send(RecordKey::NULL, item).await?;
    }
    Ok(())
}

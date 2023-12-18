#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nc = nats::asynk::connect("localhost").await.unwrap();

    nc.publish("my.subject.tofluvio", "Hello World")
        .await
        .unwrap();

    Ok(())
}

use futures_util::StreamExt;
use redis::Client;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    println!("Launching Redis subscriber...");
    let client = Client::open("redis://redis:6379")?;
    let mut pubsub = client.get_async_pubsub().await?;
    pubsub.subscribe("my-event").await?;

    while let Some(msg) = pubsub.on_message().next().await {
        let payload: String = msg.get_payload()?;
        println!("Received message: {}", payload);
    }

    Ok(())
}

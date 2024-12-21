use futures_util::stream::StreamExt;
use redis::Client;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    // Create a Redis client
    let client = Client::open("redis://redis:6379")?;
    let pubsub = client.get_async_pubsub().await.unwrap();

    let mut stream = pubsub.into_on_message();

    while let Some(msg) = stream.next().await {
        // Process the message
        println!("Received message: {:?}", msg);
    }

    Ok(())
}

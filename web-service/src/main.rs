use axum::{extract::Json, http::StatusCode, response::IntoResponse, routing::post, Router};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Deserialize)]
struct MessagePayload {
    message: String,
}

#[derive(Serialize)]
struct ResponseBody {
    status: &'static str,
    message: &'static str,
}

#[tokio::main]
async fn main() {
    // Create the app with a POST route
    let app = Router::new().route("/publish", post(publish_message));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn publish_message(
    Json(payload): Json<MessagePayload>,
) -> Result<impl IntoResponse, StatusCode> {
    let client =
        redis::Client::open("redis://redis:6379").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut conn = client
        .get_multiplexed_async_connection()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    conn.publish("my-event", payload.message)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let body = axum::Json(ResponseBody {
        status: "success",
        message: "Message published",
    });

    Ok((StatusCode::OK, body))
}

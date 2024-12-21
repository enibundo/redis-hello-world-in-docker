use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello) // Register the route
    })
    .bind("127.0.0.1:8080")? // Bind to localhost on port 8080
    .run() // Start the server
    .await
}
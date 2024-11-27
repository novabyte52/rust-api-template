use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::get, Router};
use config::env_keys;
use dotenv::dotenv;
use std::env;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*};

mod config;
mod handlers;
mod routes;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok(); // Load environment variables from .env

    tracing_subscriber::registry().with(fmt::layer()).init(); // initialize tracing

    let app = Router::new()
        .route("/", get(root_handler))
        .nest("/health", routes::health::health_routes()) // Modular routes
        .layer(TraceLayer::new_for_http()); // Example of middleware

    let addr = format!(
        "{}:{}",
        env::var(env_keys::ADDRESS).unwrap_or_else(|_| "127.0.0.1".to_string()),
        env::var(env_keys::PORT).unwrap_or_else(|_| "3000".to_string())
    );

    let listener = TcpListener::bind(&addr)
        .await
        .expect("Unable to create TCPListener.");

    info!("Server running at {}", addr);
    axum::serve(listener, app.into_make_service()).await
}

async fn root_handler() -> impl IntoResponse {
    (StatusCode::OK, "Welcome to the API!")
}

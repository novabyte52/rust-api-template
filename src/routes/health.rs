use axum::routing::get;
use axum::Router;

pub fn health_routes() -> Router {
    Router::new().route("/", get(health_check))
}

async fn health_check() -> &'static str {
    "OK"
}

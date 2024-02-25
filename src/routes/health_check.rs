use crate::handlers::health::health_check;
use axum::{routing::get, Router};

pub fn health_check_routes() -> Router {
    Router::new().route("/health", get(health_check))
}

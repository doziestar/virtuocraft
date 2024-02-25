use crate::handlers::media_processing::process_media;
use axum::{routing::post, Router};

pub fn media_routes() -> Router {
    Router::new().route("/media", post(process_media))
}

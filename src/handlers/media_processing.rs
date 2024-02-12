use crate::services::media_service::process_media_service;

pub async fn process_media() -> impl axum::response::IntoResponse {
    process_media_service().await
}

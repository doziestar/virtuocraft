use axum::response::IntoResponse;

pub async fn health_check_handler() -> String {
    "OK".to_string()
}

pub async fn health_check() -> impl IntoResponse {
    health_check_handler().await
}

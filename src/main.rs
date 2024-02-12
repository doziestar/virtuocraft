use std::{iter::once, sync::Arc};

use axum::Router;
use http::header::{HeaderName, AUTHORIZATION, CONTENT_TYPE};
use tower::ServiceBuilder;
use tower_http::{
    add_extension::AddExtensionLayer, propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveRequestHeadersLayer, set_header::SetResponseHeaderLayer,
    trace::TraceLayer, validate_request::ValidateRequestHeaderLayer,
};

mod handlers;
mod routes;
mod services;

struct State {}

#[tokio::main]
async fn main() {
    let state = State {};

    let service = ServiceBuilder::new()
        .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(Arc::new(state)))
        .layer(PropagateHeaderLayer::new(HeaderName::from_static(
            "x-request-id",
        )))
        .layer(ValidateRequestHeaderLayer::accept("application/json"));

    let app = Router::new()
        .merge(routes::health_check_routes())
        .merge(routes::media_routes())
        .layer(service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

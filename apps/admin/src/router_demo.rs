use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError, Router};
use tower::ServiceBuilder;

use axum_middleware::{demo1::Demo1Layer, demo2::Demo2Layer};

async fn handle_error(err: BoxError) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled internal error: {err}"),
    )
}

fn register() -> Router {
    let layers = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .layer(Demo1Layer)
        .layer(Demo2Layer)
        .into_inner();

    Router::new().layer(layers)
}

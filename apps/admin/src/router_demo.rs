use axum::{
    body::Body, error_handling::HandleErrorLayer, extract::Request, http::StatusCode,
    response::Response, BoxError, Extension, Router,
};
use tower::ServiceBuilder;

use app_state::AppState;
use axum_middleware::{demo1::Demo1Layer, demo2::Demo2Layer};

async fn handle_error(err: BoxError) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled internal error: {err}"),
    )
}

fn register() -> Router {
    let state = AppState {};

    let layers = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .layer(Demo1Layer)
        // .layer(Demo2Layer)
        .layer(Extension(state)) // 扩展
        .service_fn(|req: Request<Body>| async {
            // 这里可以添加对请求的处理逻辑
            Ok::<_, BoxError>(Response::new(Body::from("Hello, world!")))
        }); // 将 Service 类型转换为 Service<Request>

    Router::new().layer(layers)
}

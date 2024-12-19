//! 路由

use std::time::Duration;

use axum::{http::StatusCode, response::IntoResponse, Extension, Router};
use axum_extra::extract::{cookie::Cookie, PrivateCookieJar};
use serde::Deserialize;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use middleware::cors::cors_layer;

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}

/// 优雅关机
pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

/// 全局应用状态
#[derive(Debug, Default, Clone, Deserialize)]
pub struct AppState {}

// go ahead and run "cargo run main.rs"
// localhost:4000 should now print out your user agent
// async fn index(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
//     String::from(user_agent.as_str())
// }

async fn check_cookie(jar: PrivateCookieJar) -> impl IntoResponse {
    if let None = jar.get("hello") {
        jar.add(Cookie::new("hello", "world"));
        // jar.remove(Cookie::from("foo"));
    }

    StatusCode::OK
}

/// 注册路由
pub fn register() -> Router {
    let state = AppState {};

    Router::new()
        // 注意中间件加载顺序: Last in, first loading
        // .wrap(ApiOperation::default())
        .layer(
            ServiceBuilder::new()
                // .layer(cors_layer()) // 为CORS添加标头的中间件
                .layer(CompressionLayer::new()) // 自动压缩响应
                .layer(TraceLayer::new_for_http()) // 高级跟踪/记录
                .layer(TimeoutLayer::new(Duration::from_secs(30))) // Timeout requests after 30 seconds
                .layer(Extension(state)),
        )

    // 接口鉴权
    // .wrap(CasbinAuth::default())
    // .wrap(SystemApiAuth::default())
    // .wrap(OpenApiAuth::default())
    // .wrap(ContextMiddleware::default())
    // <<< 中间件 <<<
    // .merge( HealthRouter::register()) // 健康检查
    // .nest("/v1", LocationRouter::register())
}

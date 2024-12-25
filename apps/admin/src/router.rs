//! 路由

use std::time::Duration;

use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError, Extension, Router};
use tokio::signal;
use tower::ServiceBuilder;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::{
    compression::CompressionLayer, limit::RequestBodyLimitLayer, timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::warn;

use app_state::AppState;
use axum_context::ContextLayer;
use middleware::cors::cors_layer;
use middleware::demo::TimeoutLayer2;
use service_hub::public::HealthRouter;

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    warn!("No route {}", uri);
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

/// 注册路由
pub fn register() -> Router {
    let state = AppState {};

    // 速率限制
    //允许每个IP地址最多有五个请求的突发, 每两秒钟补充一种元素
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(5)
            .finish()
            .unwrap(),
    );

    Router::new()
        // 注意中间件加载顺序: Last in, first loading
        // .wrap(ApiOperation::default())
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    // because Axum uses infallible errors, you must handle your custom error type from your middleware here
                    StatusCode::BAD_REQUEST
                }))
                .layer(TimeoutLayer2::new(std::time::Duration::from_secs(5))) // demo
                .layer(ContextLayer::new()) // 上下文
                .layer(CompressionLayer::new()) // 自动压缩响应
                .layer(TraceLayer::new_for_http()) // 高级跟踪/记录
                .layer(TimeoutLayer::new(Duration::from_secs(30))) // Timeout requests after 30 seconds
                .layer(GovernorLayer {
                    config: governor_conf.into(),
                }) // 速率限制
                .layer(Extension(state)),
        )
        .layer(RequestBodyLimitLayer::new(4096)) // 限制了传入请求的大小，防止试图通过大量请求压垮服务器的攻击
        .layer(cors_layer()) // 为CORS添加标头的中间件
        .merge(HealthRouter::register()) // 健康检查
                                         // 接口鉴权
                                         // .wrap(CasbinAuth::default())
                                         // .wrap(SystemApiAuth::default())
                                         // .wrap(OpenApiAuth::default())
                                         // .wrap(ContextMiddleware::default())
                                         // <<< 中间件 <<<
                                         // .merge( HealthRouter::register()) // 健康检查
                                         // .nest("/v1", LocationRouter::register())
}

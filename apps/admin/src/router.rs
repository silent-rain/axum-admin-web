//! 路由

use std::{sync::Arc, time::Duration};

use axum::{extract::DefaultBodyLimit, Router};
use tokio::signal;
use tower::ServiceBuilder;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::{
    compression::CompressionLayer,
    limit::RequestBodyLimitLayer,
    request_id::MakeRequestUuid,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};
use tracing::warn;

use axum_context::ContextLayer;
use axum_middleware::{
    // api_operation_log::ApiOperationLogLayer,
    casbin_auth::CasbinAuthLayer,
    cors::cors_layer,
    openapi_auth::OpenApiAuthLayer,
    system_api_auth::SystemApiAuthLayer,
    template2::Template2Layer,
};
use service_hub::{
    auth::AuthRouter, initialize::InitializeRouter, log::LogRouter,
    organization::OrganizationRouter, permission::PermissionRouter, public::HealthRouter,
    schedule::ScheduleRouter, system::SystemRouter, template::TemplateRouter, user::UserRouter,
};

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
    // 速率限制
    //允许每个IP地址最多有五个请求的突发, 每两秒钟补充一种元素
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(5)
            .finish()
            .expect("init governor config failed"),
    );
    let governor_layer = GovernorLayer {
        config: governor_conf,
    };

    // 注意中间件加载顺序: Last in, first loading
    let layers = ServiceBuilder::new()
        // make sure to set request ids before the request reaches `TraceLayer`
        .set_x_request_id(MakeRequestUuid)
        // .layer(HandleErrorLayer::new(handle_error)) // 自定义错误类型需要添加该中间件
        .layer(
            // set request_id log requests and responses
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_response(DefaultOnResponse::new().include_headers(true)),
        ) // 高级跟踪/记录
        .layer(DefaultBodyLimit::disable()) // Disable the default limit
        .layer(RequestBodyLimitLayer::new(250 * 1024 * 1024)) //250mb, 限制了传入请求的大小，防止试图通过大量请求压垮服务器的攻击
        .layer(CompressionLayer::new()) // 自动压缩响应
        .layer(governor_layer) // 速率限制
        .layer(TimeoutLayer::new(Duration::from_secs(30))) // Timeout requests after 30 seconds
        .layer(cors_layer()) // 为CORS添加标头的中间件
        .layer(ContextLayer::new()) // 上下文
        // .layer(ApiOperationLogLayer) // Api 操作日志中间件
        .layer(SystemApiAuthLayer) // 系统接口权限中间件
        .layer(OpenApiAuthLayer) // OpenApi权限中间件
        .layer(CasbinAuthLayer) // RBAC 鉴权
        // .layer(Template2Layer)
        // propagate the header to the response before the response reaches `TraceLayer`
        .propagate_x_request_id();

    Router::new()
        .merge(HealthRouter::register()) // 健康检查
        .merge(AuthRouter::register()) // 认证管理
        .merge(UserRouter::register()) // 用户管理
        .merge(OrganizationRouter::register()) // 组织管理
        .merge(PermissionRouter::register()) // 权限管理
        .merge(SystemRouter::register()) // 系统管理
        .merge(ScheduleRouter::register()) // 定时任务管理
        .merge(LogRouter::register()) // 日志管理
        .merge(InitializeRouter::register()) // 库表资源初始化管理
        .merge(TemplateRouter::register()) // 模板管理
        .layer(layers)
}

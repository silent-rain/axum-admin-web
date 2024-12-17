//! 路由集散处, 将各个模块的路由在此处进行注册。
use context::ContextMiddleware;
use middleware::{
    api_operation::ApiOperation, casbin_auth::CasbinAuth, openapi_auth::OpenApiAuth,
    system_api_auth::SystemApiAuth,
};
use service_hub::{
    auth::AuthRouter, log::LogRouter, organization::OrganizationRouter,
    permission::PermissionRouter, public::HealthRouter, schedule::ScheduleRouter,
    system::SystemRouter, user::UserRouter,
};

use actix_web::{dev::HttpServiceFactory, web};
use tracing_actix_web::TracingLogger;

/// 注册路由
///
/// Service Hub Module: [`service_hub`]
pub fn register() -> impl HttpServiceFactory {
    web::scope("/api/v1")
        // >>> 中间件 >>>
        // 注意中间件加载顺序: Last in, first loading
        .wrap(ApiOperation::default())
        .wrap(TracingLogger::default())
        .wrap(middleware::cors::wrap_cors())
        // 接口鉴权
        .wrap(CasbinAuth::default())
        .wrap(SystemApiAuth::default())
        .wrap(OpenApiAuth::default())
        // 上下文中间件
        .wrap(ContextMiddleware::default())
        // <<< 中间件 <<<
        // 健康检查
        .service(HealthRouter::register())
        // 认证管理
        .service(AuthRouter::register())
        // 后台管理接口
        .service(
            web::scope("/admin")
                // 用户管理
                .service(UserRouter::admin_register())
                // 组织管理
                .service(OrganizationRouter::admin_register())
                // 权限管理
                .service(PermissionRouter::admin_register())
                // 系统管理
                .service(SystemRouter::admin_register())
                // 定时任务管理
                .service(ScheduleRouter::admin_register())
                // 日志管理
                .service(LogRouter::admin_register()),
        )
}

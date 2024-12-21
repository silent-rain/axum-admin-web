//! 后台管理 WEB 服务

use crate::controller::admin_web_site::AdminWebSiteController;

use axum::{routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

/// 路由
pub struct AdminWebSiteRouter;

impl AdminWebSiteRouter {
    /// 注册`后台管理 WEB 服务`路由
    pub fn register() -> Router {
        Router::new()
            // 内嵌服务
            // http://0.0.0.0:3000/admin
            .route_service(
                "/embed-admin",
                get(AdminWebSiteController::index), // 后台 WEB 静态资源服务-首页
            )
            .route_service(
                "/embed-admin/*path",
                get(AdminWebSiteController::static_dir), // 后台 WEB 静态资源服务-静态资源
            )
            // http://0.0.0.0:3000/admin/
            .nest_service(
                "/admin",
                ServeDir::new("dist")
                    .append_index_html_on_directories(true)
                    .not_found_service(ServeFile::new("dist/index.html")),
            )
    }
}

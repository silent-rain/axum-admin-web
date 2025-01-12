//! 用户session管理

use crate::controller::user_session::UserSessionController;

use axum::{
    routing::{get, put},
    Router,
};

/// 路由器
pub struct UserSessionRouter;

impl UserSessionRouter {
    /// 注册`用户session管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/phones",
            Router::new()
                .route(
                    "/",
                    get(UserSessionController::list).post(UserSessionController::create),
                )
                .route(
                    "/{id}",
                    get(UserSessionController::info)
                        .put(UserSessionController::update)
                        .delete(UserSessionController::delete),
                )
                .route("/{id}/status", put(UserSessionController::update_status)),
        )
    }
}

//! 用户信息管理

use axum::{
    Router,
    routing::{get, put},
};

use crate::controller::user_base::UserBaseController;

/// 路由器
pub struct UserBaseRouter;

impl UserBaseRouter {
    /// 注册`用户信息管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/base",
            Router::new()
                .route(
                    "/",
                    get(UserBaseController::list).post(UserBaseController::create),
                )
                .route(
                    "/{id}",
                    get(UserBaseController::info)
                        .put(UserBaseController::update)
                        .delete(UserBaseController::delete),
                )
                .route("/{id}/status", put(UserBaseController::update_status))
                .route("/profile", get(UserBaseController::profile))
                .route("/{id}/roles", get(UserBaseController::roles))
                .route("/check-username", get(UserBaseController::check_username)),
        )
    }
}

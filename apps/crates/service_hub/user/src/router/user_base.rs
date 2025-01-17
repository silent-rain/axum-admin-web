//! 用户信息管理

use crate::controller::user_base::UserBaseController;

use axum::{
    routing::{get, put},
    Router,
};

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
                .route(
                    "/{id}/share-code",
                    put(UserBaseController::update_share_code),
                )
                .route("/{id}/profile", get(UserBaseController::profile))
                .route("/{id}/roles", get(UserBaseController::roles))
                .route("/check-username", get(UserBaseController::check_username)),
        )
    }
}

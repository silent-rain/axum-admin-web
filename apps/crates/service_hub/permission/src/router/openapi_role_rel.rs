//! OpenApi接口角色关系管理

use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::controller::openapi_role_rel::OpenapiRoleRelController;

/// 路由器
pub struct OpenapiRoleRelRouter;

impl OpenapiRoleRelRouter {
    /// 注册`OpenApi接口角色关系管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/openapi-role-rels",
            Router::new()
                .route("/", get(OpenapiRoleRelController::list))
                .route(
                    "/batch_create",
                    post(OpenapiRoleRelController::batch_create),
                )
                .route(
                    "/batch_delete",
                    delete(OpenapiRoleRelController::batch_delete),
                ),
        )
    }
}

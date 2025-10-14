//! 菜单管理

use axum::{
    Router,
    routing::{get, put},
};

use crate::controller::menu::MenuController;

/// 路由器
pub struct MenuRouter;

impl MenuRouter {
    /// 注册`菜单管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/menus",
            Router::new()
                .route("/", get(MenuController::list).post(MenuController::create))
                .route("/tree", get(MenuController::tree))
                .route("/children", get(MenuController::children))
                .route(
                    "/{id}",
                    get(MenuController::info)
                        .put(MenuController::update)
                        .delete(MenuController::delete),
                )
                .route("/{id}/status", put(MenuController::update_status)),
        )
    }
}

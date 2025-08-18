//! 字典数据管理

use axum::{
    Router,
    routing::{get, put},
};

use crate::controller::dict_data::DictDataController;

/// 路由器
pub struct DictDataRouter;

impl DictDataRouter {
    /// 注册`字典数据管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/dict-datas",
            Router::new()
                .route(
                    "/",
                    get(DictDataController::list).post(DictDataController::create),
                )
                .route(
                    "/{id}",
                    get(DictDataController::info)
                        .put(DictDataController::update)
                        .delete(DictDataController::delete),
                )
                .route("/{id}/status", put(DictDataController::update_status)),
        )
    }
}

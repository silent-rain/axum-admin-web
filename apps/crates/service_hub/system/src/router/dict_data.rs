//! 字典数据管理

use crate::controller::dict_data::DictDataController;

use axum::{
    routing::{get, put},
    Router,
};

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

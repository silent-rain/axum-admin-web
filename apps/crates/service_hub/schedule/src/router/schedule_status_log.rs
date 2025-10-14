//! 任务调度状态日志管理

use axum::{
    Router,
    routing::{get, put},
};

use crate::controller::schedule_status_log::ScheduleStatusLogController;

/// 路由器
pub struct ScheduleStatusLogRouter;

impl ScheduleStatusLogRouter {
    /// 注册`任务调度状态日志管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/status-logs",
            Router::new()
                .route(
                    "/",
                    get(ScheduleStatusLogController::list)
                        .post(ScheduleStatusLogController::create),
                )
                .route(
                    "/{id}",
                    get(ScheduleStatusLogController::info)
                        .put(ScheduleStatusLogController::update)
                        .delete(ScheduleStatusLogController::delete),
                )
                .route(
                    "/{id}/status",
                    put(ScheduleStatusLogController::update_status),
                ),
        )
    }
}

//! 任务调度事件日志管理

use axum::{Router, routing::get};

use crate::controller::schedule_event_log::ScheduleEventLogController;

/// 路由器
pub struct ScheduleEventLogRouter;

impl ScheduleEventLogRouter {
    /// 注册`任务调度事件日志管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/event-logs",
            Router::new()
                .route(
                    "/",
                    get(ScheduleEventLogController::list).post(ScheduleEventLogController::create),
                )
                .route(
                    "/{id}",
                    get(ScheduleEventLogController::info)
                        .delete(ScheduleEventLogController::delete),
                ),
        )
    }
}

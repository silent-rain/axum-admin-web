//! 任务调度作业管理

use axum::{
    routing::{get, put},
    Router,
};

use crate::controller::schedule_job::ScheduleJobController;

/// 路由器
pub struct ScheduleJobRouter;

impl ScheduleJobRouter {
    /// 注册`任务调度作业管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/jobs",
            Router::new()
                .route(
                    "/",
                    get(ScheduleJobController::list).post(ScheduleJobController::create),
                )
                .route(
                    "/{id}",
                    get(ScheduleJobController::info)
                        .put(ScheduleJobController::update)
                        .delete(ScheduleJobController::delete),
                )
                .route("/{id}/status", put(ScheduleJobController::update_status)),
        )
    }
}

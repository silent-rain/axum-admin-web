//! 路由层

use axum::Router;
pub mod schedule_event_log;
pub mod schedule_job;
pub mod schedule_status_log;

/// 路由器
pub struct ScheduleRouter;

impl ScheduleRouter {
    /// 注册`任务调度作业管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/schedule",
            Router::new()
                .merge(schedule_job::ScheduleJobRouter::register()) // 任务调度作业管理
                .merge(schedule_status_log::ScheduleStatusLogRouter::register()) //任务调度状态日志管理板
                .merge(schedule_event_log::ScheduleEventLogRouter::register()), // 任务调度事件日志管理
        )
    }
}

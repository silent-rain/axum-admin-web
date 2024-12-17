//! 任务调度作业管理

use crate::controller::schedule_job::ScheduleJobController;

use actix_web::{web, Scope};

/// 路由器
pub struct ScheduleJobRouter;

impl ScheduleJobRouter {
    /// 注册`任务调度作业管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/jobs")
            .route("", web::get().to(ScheduleJobController::list))
            .route("/{id}", web::get().to(ScheduleJobController::info))
            .route("", web::post().to(ScheduleJobController::add))
            .route("/{id}", web::put().to(ScheduleJobController::update))
            .route("/{id}/status", web::put().to(ScheduleJobController::status))
            .route("/{id}", web::delete().to(ScheduleJobController::delete))
    }
}

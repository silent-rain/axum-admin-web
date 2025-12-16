//! 任务调度作业管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};
use inject::AInjectProvider;

use crate::{
    dto::schedule_job::{
        CreateScheduleJobReq, DeleteScheduleJobReq, GetScheduleJobReq, GetScheduleJobsReq,
        UpdateScheduleJobReq, UpdateScheduleJobStatusReq,
    },
    service::schedule_job::ScheduleJobService,
};

/// 控制器
pub struct ScheduleJobController;

impl ScheduleJobController {
    /// 获取任务调度列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetScheduleJobsReq>,
    ) -> Responder<()> {
        let schedule_job_service: ScheduleJobService = provider.provide();
        let (results, total) = schedule_job_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取任务调度信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetScheduleJobReq>,
    ) -> Responder<()> {
        let schedule_job_service: ScheduleJobService = provider.provide();
        let result = schedule_job_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加任务调度
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateScheduleJobReq>,
    ) -> Responder<()> {
        let schedule_job_service: ScheduleJobService = provider.provide();
        let _result = schedule_job_service.create(req).await?;

        Ok(Response::ok())
    }

    /// 更新任务调度
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateScheduleJobReq>,
    ) -> Responder<()> {
        let schedule_job_service: ScheduleJobService = provider.provide();
        let _result = schedule_job_service.update(req).await?;

        Ok(Response::ok())
    }
    /// 更新任务调度状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateScheduleJobStatusReq>,
    ) -> Responder<()> {
        let schedule_job_service: ScheduleJobService = provider.provide();
        schedule_job_service.update_status(req).await?;

        Ok(Response::ok())
    }

    /// 删除任务调度
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteScheduleJobReq>,
    ) -> Responder<()> {
        let schedule_job_service: ScheduleJobService = provider.provide();
        let _result = schedule_job_service.delete(req).await?;

        Ok(Response::ok())
    }
}

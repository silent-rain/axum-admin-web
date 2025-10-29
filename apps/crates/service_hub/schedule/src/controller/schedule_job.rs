//! 任务调度作业管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};
use inject::AInjectProvider;

use crate::{
    dto::schedule_job::{
        CreateScheduleJobReq, CreateScheduleJobResp, DeleteScheduleJobReq, DeleteScheduleJobResp,
        GetScheduleJobReq, GetScheduleJobResp, GetScheduleJobsReq, GetScheduleJobsResp,
        UpdateScheduleJobReq, UpdateScheduleJobResp, UpdateScheduleJobStatusReq,
        UpdateScheduleJobStatusResp,
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
    ) -> Responder<GetScheduleJobsResp> {
        let schedule_job_service: ScheduleJobService = provider.provide();
        let (results, total) = schedule_job_service.list(req).await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取任务调度信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetScheduleJobReq>,
    ) -> Responder<GetScheduleJobResp> {
        let schedule_job_service: ScheduleJobService = provider.provide();
        let result = schedule_job_service.info(req).await?;

        let resp = Response::data(result.into());
        Ok(resp)
    }

    /// 添加任务调度
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateScheduleJobReq>,
    ) -> Responder<CreateScheduleJobResp> {
        let schedule_job_service: ScheduleJobService = provider.provide();
        let _result = schedule_job_service.create(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 更新任务调度
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateScheduleJobReq>,
    ) -> Responder<UpdateScheduleJobResp> {
        let schedule_job_service: ScheduleJobService = provider.provide();
        let _result = schedule_job_service.update(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }
    /// 更新任务调度状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateScheduleJobStatusReq>,
    ) -> Responder<UpdateScheduleJobStatusResp> {
        let schedule_job_service: ScheduleJobService = provider.provide();
        schedule_job_service.update_status(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 删除任务调度
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteScheduleJobReq>,
    ) -> Responder<DeleteScheduleJobResp> {
        let schedule_job_service: ScheduleJobService = provider.provide();
        let _result = schedule_job_service.delete(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }
}

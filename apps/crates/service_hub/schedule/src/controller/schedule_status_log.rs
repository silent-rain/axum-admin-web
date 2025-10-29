//! 任务调度状态日志管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};
use inject::AInjectProvider;

use crate::{
    dto::schedule_status_log::{
        CreateScheduleStatusLogReq, CreateScheduleStatusLogResp, DeleteScheduleStatusLogReq,
        DeleteScheduleStatusLogResp, GetScheduleStatusLogReq, GetScheduleStatusLogResp,
        GetScheduleStatusLogsReq, GetScheduleStatusLogsResp, UpdateScheduleStatusLogReq,
        UpdateScheduleStatusLogResp, UpdateScheduleStatusLogSatausReq,
        UpdateScheduleStatusLogSatausResp,
    },
    service::schedule_status_log::ScheduleStatusLogService,
};

/// 控制器
pub struct ScheduleStatusLogController;

impl ScheduleStatusLogController {
    /// 获取任务调度状态日志列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetScheduleStatusLogsReq>,
    ) -> Responder<GetScheduleStatusLogsResp> {
        let schedule_status_log_service: ScheduleStatusLogService = provider.provide();
        let (results, total) = schedule_status_log_service.list(req).await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取字典数据信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetScheduleStatusLogReq>,
    ) -> Responder<GetScheduleStatusLogResp> {
        let schedule_status_log_service: ScheduleStatusLogService = provider.provide();
        let result = schedule_status_log_service.info(req).await?;

        let resp = Response::data(result.into());
        Ok(resp)
    }

    /// 添加字典数据
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateScheduleStatusLogReq>,
    ) -> Responder<CreateScheduleStatusLogResp> {
        let schedule_status_log_service: ScheduleStatusLogService = provider.provide();
        let _result = schedule_status_log_service.create(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 更新字典数据
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateScheduleStatusLogReq>,
    ) -> Responder<UpdateScheduleStatusLogResp> {
        let schedule_status_log_service: ScheduleStatusLogService = provider.provide();
        let _result = schedule_status_log_service.update(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 更新字典数据状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateScheduleStatusLogSatausReq>,
    ) -> Responder<UpdateScheduleStatusLogSatausResp> {
        let schedule_status_log_service: ScheduleStatusLogService = provider.provide();
        schedule_status_log_service.update_status(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 删除字典数据
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteScheduleStatusLogReq>,
    ) -> Responder<DeleteScheduleStatusLogResp> {
        let schedule_status_log_service: ScheduleStatusLogService = provider.provide();
        let _result = schedule_status_log_service.delete(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }
}

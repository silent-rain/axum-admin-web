//! 任务调度事件日志管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};
use inject::AInjectProvider;

use crate::{
    dto::schedule_event_log::{
        CreateScheduleEventLogReq, CreateScheduleEventLogResp, DeleteScheduleEventLogReq,
        DeleteScheduleEventLogResp, GetScheduleEventLogReq, GetScheduleEventLogResp,
        GetScheduleEventLogsReq, GetScheduleEventLogsResp,
    },
    service::schedule_event_log::ScheduleEventLogService,
};
/// 控制器
pub struct ScheduleEventLogController;

impl ScheduleEventLogController {
    /// 获取任务调度事件日志列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetScheduleEventLogsReq>,
    ) -> Responder<GetScheduleEventLogsResp> {
        let schedule_event_log_service: ScheduleEventLogService = provider.provide();
        let (results, total) = schedule_event_log_service.list(req).await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取任务调度事件日志的详细信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetScheduleEventLogReq>,
    ) -> Responder<GetScheduleEventLogResp> {
        let schedule_event_log_service: ScheduleEventLogService = provider.provide();
        let result = schedule_event_log_service.info(req).await?;

        let resp = Response::data(result.into());
        Ok(resp)
    }

    /// 添加任务调度事件日志
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateScheduleEventLogReq>,
    ) -> Responder<CreateScheduleEventLogResp> {
        let schedule_event_log_service: ScheduleEventLogService = provider.provide();
        let _result = schedule_event_log_service.create(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 删除任务调度事件日志
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteScheduleEventLogReq>,
    ) -> Responder<DeleteScheduleEventLogResp> {
        let schedule_event_log_service: ScheduleEventLogService = provider.provide();
        let _result = schedule_event_log_service.delete(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }
}

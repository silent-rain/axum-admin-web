//! 系统日志

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};
use inject::AInjectProvider;

use crate::{
    dto::system_log::{
        CreateSystemLogReq, CreateSystemLogResp, DeleteSystemLogReq, DeleteSystemLogResp,
        GetSystemLogReq, GetSystemLogResp, GetSystemLogsReq, GetSystemLogsResp,
    },
    service::system_log::SystemLogService,
};

/// 控制器
pub struct SystemLogController;

impl SystemLogController {
    /// 获取系统日志列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetSystemLogsReq>,
    ) -> Responder<GetSystemLogsResp> {
        let system_service: SystemLogService = provider.provide();
        let (results, total) = system_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取系统日志的详细信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetSystemLogReq>,
    ) -> Responder<GetSystemLogResp> {
        let system_service: SystemLogService = provider.provide();
        let result = system_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加系统日志
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateSystemLogReq>,
    ) -> Responder<CreateSystemLogResp> {
        let system_service: SystemLogService = provider.provide();
        let _result = system_service.create(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 删除系统日志
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteSystemLogReq>,
    ) -> Responder<DeleteSystemLogResp> {
        let system_service: SystemLogService = provider.provide();
        let _result = system_service.delete(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}

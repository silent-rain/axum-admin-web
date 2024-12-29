//! 系统日志

use crate::{
    dto::system::{
        CreateSystemReq, CreateSystemResp, DeleteSystemReq, DeleteSystemResp, GetSystemReq,
        GetSystemResp, GetSystemsReq, GetSystemsResp,
    },
    service::system::SystemService,
};

use axum::{extract::Query, Extension, Json};
use inject::AInjectProvider;
use response::{Responder, Response};

/// 控制器
pub struct SystemController;

impl SystemController {
    /// 获取系统日志列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetSystemsReq>,
    ) -> Responder<GetSystemsResp> {
        let system_service: SystemService = provider.provide();
        let (results, total) = system_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取系统日志的详细信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetSystemReq>,
    ) -> Responder<GetSystemResp> {
        let system_service: SystemService = provider.provide();
        let result = system_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加系统日志
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateSystemReq>,
    ) -> Responder<CreateSystemResp> {
        let system_service: SystemService = provider.provide();
        let _result = system_service.create(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 删除系统日志
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteSystemReq>,
    ) -> Responder<DeleteSystemResp> {
        let system_service: SystemService = provider.provide();
        let _result = system_service.delete(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}

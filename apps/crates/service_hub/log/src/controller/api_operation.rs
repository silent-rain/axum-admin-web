//! API操作日志

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};
use inject::AInjectProvider;

use crate::{
    dto::api_operation::{
        CreateApiOperationReq, CreateApiOperationResp, DeleteApiOperationReq,
        DeleteApiOperationResp, GetApiOperationReq, GetApiOperationResp, GetApiOperationsReq,
        GetApiOperationsResp,
    },
    service::api_operation::ApiOperationService,
};

/// 控制器
pub struct ApiOperationController;

impl ApiOperationController {
    /// 获取API操作日志列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetApiOperationsReq>,
    ) -> Responder<GetApiOperationsResp> {
        let api_operation_service: ApiOperationService = provider.provide();
        let (results, total) = api_operation_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取API操作日志信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetApiOperationReq>,
    ) -> Responder<GetApiOperationResp> {
        let api_operation_service: ApiOperationService = provider.provide();
        let result = api_operation_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加API操作日志
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateApiOperationReq>,
    ) -> Responder<CreateApiOperationResp> {
        let api_operation_service: ApiOperationService = provider.provide();
        let _result = api_operation_service.create(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 删除API操作日志
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteApiOperationReq>,
    ) -> Responder<DeleteApiOperationResp> {
        let api_operation_service: ApiOperationService = provider.provide();
        let _result = api_operation_service.delete(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}

//! OpenApi接口管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::openapi::{
        CreateOpenapiReq, CreateOpenapiResp, DeleteOpenapiReq, DeleteOpenapiResp, GetOpenapiReq,
        GetOpenapiResp, GetOpenapiTreeReq, GetOpenapiTreeResp, GetOpenapisReq, GetOpenapisResp,
        UpdateOpenapiReq, UpdateOpenapiResp, UpdateOpenapiStatusReq, UpdateOpenapiStatusResp,
    },
    service::openapi::OpenapiService,
};

/// 控制器
pub struct OpenapiController;

impl OpenapiController {
    /// 获取OpenApi接口列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetOpenapisReq>,
    ) -> Responder<GetOpenapisResp> {
        let openapi_service: OpenapiService = provider.provide();
        let (results, total) = openapi_service.list(req).await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取OpenApi接口树列表
    pub async fn tree(
        Extension(provider): Extension<AInjectProvider>,
        Query(_req): Query<GetOpenapiTreeReq>,
    ) -> Responder<GetOpenapiTreeResp> {
        let openapi_service: OpenapiService = provider.provide();
        let result = openapi_service.tree().await?;

        let resp = Response::data(result.into());
        Ok(resp)
    }

    /// 获取OpenApi接口信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetOpenapiReq>,
    ) -> Responder<GetOpenapiResp> {
        let openapi_service: OpenapiService = provider.provide();
        let result = openapi_service.info(req).await?;

        let resp = Response::data(result.into());
        Ok(resp)
    }

    /// 添加OpenApi接口
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(data): Json<CreateOpenapiReq>,
    ) -> Responder<CreateOpenapiResp> {
        let openapi_service: OpenapiService = provider.provide();
        let _result = openapi_service.create(data).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 更新OpenApi接口
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateOpenapiReq>,
    ) -> Responder<UpdateOpenapiResp> {
        let openapi_service: OpenapiService = provider.provide();
        let _result = openapi_service.update(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 更新OpenApi接口状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateOpenapiStatusReq>,
    ) -> Responder<UpdateOpenapiStatusResp> {
        let openapi_service: OpenapiService = provider.provide();
        openapi_service.update_status(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 删除OpenApi接口
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteOpenapiReq>,
    ) -> Responder<DeleteOpenapiResp> {
        let openapi_service: OpenapiService = provider.provide();
        let _result = openapi_service.delete(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }
}

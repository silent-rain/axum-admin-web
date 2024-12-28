//! OpenApi接口角色关系管理

use crate::{
    dto::openapi_role_rel::{
        BatchCreateOpenapiRoleRelReq, BatchCreateOpenapiRoleRelResp, BatchDeleteOpenapiRoleRelReq,
        BatchDeleteOpenapiRoleRelResp, GetOpenapiRoleRelsReq, GetOpenapiRoleRelsResp,
    },
    service::openapi_role_rel::OpenapiRoleRelService,
};

use axum::{extract::Query, Extension, Json};
use inject::AInjectProvider;
use response::{Responder, Response};

/// 控制器
pub struct OpenapiRoleRelController;

impl OpenapiRoleRelController {
    /// 获取OpenApi接口角色关系列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetOpenapiRoleRelsReq>,
    ) -> Responder<GetOpenapiRoleRelsResp> {
        let openapi_role_rel_service: OpenapiRoleRelService = provider.provide();
        let (results, total) = openapi_role_rel_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 批量创建OpenApi接口角色关系
    pub async fn batch_create(
        Extension(provider): Extension<AInjectProvider>,
        Json(data): Json<BatchCreateOpenapiRoleRelReq>,
    ) -> Responder<BatchCreateOpenapiRoleRelResp> {
        let openapi_role_rel_service: OpenapiRoleRelService = provider.provide();
        let _result = openapi_role_rel_service.batch_create(data).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 批量删除OpenApi接口角色关系
    pub async fn batch_delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(data): Json<BatchDeleteOpenapiRoleRelReq>,
    ) -> Responder<BatchDeleteOpenapiRoleRelResp> {
        let openapi_role_rel_service: OpenapiRoleRelService = provider.provide();
        let _result = openapi_role_rel_service
            .batch_delete(data.ids.clone())
            .await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}

//! 令牌角色关系管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::token_role_rel::{
        BatchCreateTokenRoleRelReq, BatchCreateTokenRoleRelResp, BatchDeleteTokenRoleRelReq,
        BatchDeleteTokenRoleRelResp, GetTokenRoleRelsReq, GetTokenRoleRelsResp,
    },
    service::token_role_rel::TokenRoleRelService,
};

/// 控制器
pub struct TokenRoleRelController;

impl TokenRoleRelController {
    /// 获取令牌角色关系列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetTokenRoleRelsReq>,
    ) -> Responder<GetTokenRoleRelsResp> {
        let token_role_rel_service: TokenRoleRelService = provider.provide();
        let (results, total) = token_role_rel_service.list(req).await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 批量创建令牌角色关系
    pub async fn batch_create(
        Extension(provider): Extension<AInjectProvider>,
        Json(data): Json<BatchCreateTokenRoleRelReq>,
    ) -> Responder<BatchCreateTokenRoleRelResp> {
        let token_role_rel_service: TokenRoleRelService = provider.provide();
        let _result = token_role_rel_service.batch_create(data).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 批量删除令牌角色关系
    pub async fn batch_delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(data): Json<BatchDeleteTokenRoleRelReq>,
    ) -> Responder<BatchDeleteTokenRoleRelResp> {
        let token_role_rel_service: TokenRoleRelService = provider.provide();
        let _result = token_role_rel_service
            .batch_delete(data.ids.clone())
            .await?;

        let resp = Response::ok();
        Ok(resp)
    }
}

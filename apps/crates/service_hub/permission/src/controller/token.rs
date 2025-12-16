//! 令牌管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::token::{
        CreateTokenReq, DeleteTokenReq, GetTokenReq, GetTokenResp, GetTokensReq, GetTokensResp,
        UpdateTokenReq, UpdateTokenStatusReq,
    },
    service::token::TokenService,
};

/// 控制器
pub struct TokenController;

impl TokenController {
    /// 获取令牌列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetTokensReq>,
    ) -> Responder<GetTokensResp> {
        let token_service: TokenService = provider.provide();
        let (results, total) = token_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取令牌信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetTokenReq>,
    ) -> Responder<GetTokenResp> {
        let token_service: TokenService = provider.provide();
        let result = token_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加令牌
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(data): Json<CreateTokenReq>,
    ) -> Responder<()> {
        let token_service: TokenService = provider.provide();
        let _result = token_service.create(data).await?;

        Ok(Response::ok())
    }

    /// 更新令牌
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateTokenReq>,
    ) -> Responder<()> {
        let token_service: TokenService = provider.provide();
        let _result = token_service.update(req).await?;

        Ok(Response::ok())
    }

    /// 更新令牌状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateTokenStatusReq>,
    ) -> Responder<()> {
        let token_service: TokenService = provider.provide();
        token_service.update_status(req).await?;

        Ok(Response::ok())
    }

    /// 删除令牌
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteTokenReq>,
    ) -> Responder<()> {
        let token_service: TokenService = provider.provide();
        let _result = token_service.delete(req).await?;

        Ok(Response::ok())
    }
}

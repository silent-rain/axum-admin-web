//! 用户session管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::user_session::{
        CreateUserSessionReq, CreateUserSessionResp, DeleteUserSessionReq, DeleteUserSessionResp,
        GetUserSessionReq, GetUserSessionResp, GetUserSessionsReq, GetUserSessionsResp,
        UpdateUserSessionReq, UpdateUserSessionResp, UpdateUserSessionStatusReq,
        UpdateUserSessionStatusResp,
    },
    service::user_session::UserSessionService,
};

/// 控制器
pub struct UserSessionController;

impl UserSessionController {
    /// 获取用户session列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetUserSessionsReq>,
    ) -> Responder<GetUserSessionsResp> {
        let user_session_service: UserSessionService = provider.provide();
        let (results, total) = user_session_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取用户session信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetUserSessionReq>,
    ) -> Responder<GetUserSessionResp> {
        let user_session_service: UserSessionService = provider.provide();
        let result = user_session_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加用户session
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(data): Json<CreateUserSessionReq>,
    ) -> Responder<CreateUserSessionResp> {
        let user_session_service: UserSessionService = provider.provide();
        let _result = user_session_service.create(data).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新用户session
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateUserSessionReq>,
    ) -> Responder<UpdateUserSessionResp> {
        let user_session_service: UserSessionService = provider.provide();
        let _result = user_session_service.update(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新用户session状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateUserSessionStatusReq>,
    ) -> Responder<UpdateUserSessionStatusResp> {
        let user_session_service: UserSessionService = provider.provide();
        user_session_service.update_status(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 删除用户session
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteUserSessionReq>,
    ) -> Responder<DeleteUserSessionResp> {
        let user_session_service: UserSessionService = provider.provide();
        let _result = user_session_service.delete(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}

//! 登陆日志管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::user_login_log::{
        CreateUserLoginLogReq, CreateUserLoginLogResp, GetUserLoginLogReq, GetUserLoginLogResp,
        GetUserLoginLogsReq, GetUserLoginLogsResp,
    },
    service::user_login_log::UserLoginLogService,
};

/// 控制器
pub struct UserLoginLogController;

impl UserLoginLogController {
    /// 获取登录日志列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetUserLoginLogsReq>,
    ) -> Responder<GetUserLoginLogsResp> {
        let user_login_service: UserLoginLogService = provider.provide();
        let (results, total) = user_login_service.list(req).await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取登陆日志信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetUserLoginLogReq>,
    ) -> Responder<GetUserLoginLogResp> {
        let user_login_service: UserLoginLogService = provider.provide();
        let result = user_login_service.info(req).await?;

        let resp = Response::data(result.into());
        Ok(resp)
    }

    /// 添加登陆日志
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(data): Json<CreateUserLoginLogReq>,
    ) -> Responder<CreateUserLoginLogResp> {
        let user_login_service: UserLoginLogService = provider.provide();
        let _result = user_login_service.create(data).await?;

        let resp = Response::ok();
        Ok(resp)
    }
}

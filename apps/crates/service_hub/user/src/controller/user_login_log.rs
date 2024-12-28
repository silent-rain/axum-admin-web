//! 登陆日志管理

use crate::{
    dto::user_login_log::{
        CreateUserLoginLogReq, CreateUserLoginLogResp, GetUserLoginLogReq, GetUserLoginLogResp,
        GetUserLoginLogsReq, GetUserLoginLogsResp, UpdateUserLoginLogReq, UpdateUserLoginLogResp,
        UpdateUserLoginLogStatusReq, UpdateUserLoginLogStatusResp,
    },
    service::user_login_log::UserLoginLogService,
};

use axum::{extract::Query, Extension, Json};
use inject::AInjectProvider;
use response::{Responder, Response};

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

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取登陆日志信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetUserLoginLogReq>,
    ) -> Responder<GetUserLoginLogResp> {
        let user_login_service: UserLoginLogService = provider.provide();
        let result = user_login_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加登陆日志
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(data): Json<CreateUserLoginLogReq>,
    ) -> Responder<CreateUserLoginLogResp> {
        let user_login_service: UserLoginLogService = provider.provide();
        let _result = user_login_service.create(data).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新登陆日志
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateUserLoginLogReq>,
    ) -> Responder<UpdateUserLoginLogResp> {
        let user_login_service: UserLoginLogService = provider.provide();
        let _result = user_login_service.update(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新登录日志状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateUserLoginLogStatusReq>,
    ) -> Responder<UpdateUserLoginLogStatusResp> {
        let user_login_service: UserLoginLogService = provider.provide();
        user_login_service.update_status(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}

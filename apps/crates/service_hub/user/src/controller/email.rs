//! 用户邮箱管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::email::{
        CreateEmailReq, CreateEmailResp, DeleteEmailReq, DeleteEmailResp, GetEmailReq,
        GetEmailResp, GetEmailsReq, GetEmailsResp, UpdateEmailReq, UpdateEmailResp,
    },
    service::email::EmailService,
};

/// 控制器
pub struct EmailController;

impl EmailController {
    /// 获取用户邮箱列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetEmailsReq>,
    ) -> Responder<GetEmailsResp> {
        let email_service: EmailService = provider.provide();
        let (results, total) = email_service.list(req).await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取用户邮箱信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetEmailReq>,
    ) -> Responder<GetEmailResp> {
        let email_service: EmailService = provider.provide();
        let result = email_service.info(req).await?;

        let resp = Response::data(result.into());
        Ok(resp)
    }

    /// 添加用户邮箱
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(data): Json<CreateEmailReq>,
    ) -> Responder<CreateEmailResp> {
        let email_service: EmailService = provider.provide();
        let _result = email_service.create(data).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 更新用户邮箱
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateEmailReq>,
    ) -> Responder<UpdateEmailResp> {
        let email_service: EmailService = provider.provide();
        let _result = email_service.update(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 删除用户邮箱
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteEmailReq>,
    ) -> Responder<DeleteEmailResp> {
        let email_service: EmailService = provider.provide();
        let _result = email_service.delete(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }
}

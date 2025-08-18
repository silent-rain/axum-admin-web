//! 用户手机号管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::phone::{
        CreatePhoneReq, CreatePhoneResp, DeletePhoneReq, DeletePhoneResp, GetPhoneReq,
        GetPhoneResp, GetPhonesReq, GetPhonesResp, UpdatePhoneReq, UpdatePhoneResp,
    },
    service::phone::PhoneService,
};

/// 控制器
pub struct PhoneController;

impl PhoneController {
    /// 获取用户手机号列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetPhonesReq>,
    ) -> Responder<GetPhonesResp> {
        let phone_service: PhoneService = provider.provide();
        let (results, total) = phone_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取用户手机号信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetPhoneReq>,
    ) -> Responder<GetPhoneResp> {
        let phone_service: PhoneService = provider.provide();
        let result = phone_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加用户手机号
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(data): Json<CreatePhoneReq>,
    ) -> Responder<CreatePhoneResp> {
        let phone_service: PhoneService = provider.provide();
        let _result = phone_service.create(data).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新用户手机号
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdatePhoneReq>,
    ) -> Responder<UpdatePhoneResp> {
        let phone_service: PhoneService = provider.provide();
        let _result = phone_service.update(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 删除用户手机号
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeletePhoneReq>,
    ) -> Responder<DeletePhoneResp> {
        let phone_service: PhoneService = provider.provide();
        let _result = phone_service.delete(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}

//! 注册

use crate::{
    dto::register::{RegisterReq, RegisterResp},
    RegisterService,
};

use axum::{Extension, Json};
use code::Error;
use entity::user::user_base;
use inject::AInjectProvider;
use response::{Responder, Response};
use tracing::error;

/// 控制器
pub struct RegisterController;

impl RegisterController {
    /// 注册用户
    pub async fn register(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<RegisterReq>,
    ) -> Responder<RegisterResp> {
        match req.register_type {
            user_base::enums::UserType::Phone => {
                if req.phone.is_none() {
                    error!("请输入手机号码");
                    return Err(Error::InvalidParameter
                        .into_msg()
                        .with_msg("请输入手机号码")
                        .into());
                }
            }
            user_base::enums::UserType::Email => {
                if req.email.is_none() {
                    error!("请输入邮箱");
                    return Err(Error::InvalidParameter
                        .into_msg()
                        .with_msg("请输入邮箱")
                        .into());
                }
            }
        }

        let register_service: RegisterService = provider.provide();
        let _result = register_service.register(req).await?;

        let resp = Response::<()>::ok().with_msg("注册成功").to_json()?;
        Ok(resp)
    }
}

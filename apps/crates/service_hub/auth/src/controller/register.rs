//! 注册

use crate::{
    dto::register::{RegisterReq, RegisterResp},
    RegisterService,
};

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json};
use inject::AInjectProvider;

/// 控制器
pub struct RegisterController;

impl RegisterController {
    /// 注册用户
    pub async fn register(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<RegisterReq>,
    ) -> Responder<RegisterResp> {
        let register_service: RegisterService = provider.provide();
        let _result = register_service.register(req).await?;

        let resp = Response::<()>::ok().with_msg("注册成功").to_json()?;
        Ok(resp)
    }
}

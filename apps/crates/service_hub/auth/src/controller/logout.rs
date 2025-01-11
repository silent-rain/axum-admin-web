//! 登出

use crate::{
    dto::logout::{LogoutReq, LogoutResp},
    service::logout::Logoutervice,
};

use axum_context::Context;
use axum_response::{Responder, Response};
use inject::AInjectProvider;

use axum_validator::{Extension, Json};

/// 控制器
pub struct LogoutController;

impl LogoutController {
    /// 登出
    pub async fn logout(
        Extension(provider): Extension<AInjectProvider>,
        ctx: Context,
        Json(_req): Json<LogoutReq>,
    ) -> Responder<LogoutResp> {
        let user_id = ctx.get_user_id();
        let user_login_id = ctx.get_user_login_id();

        let login_service: Logoutervice = provider.provide();
        login_service.logout(user_id, user_login_id).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}

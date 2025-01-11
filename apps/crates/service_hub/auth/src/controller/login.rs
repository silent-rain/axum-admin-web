//! 登陆

use std::net::SocketAddr;

use crate::{
    dto::login::{BrowserInfo, LoginReq, LoginResp},
    LoginService,
};

use axum::{extract::ConnectInfo, http::HeaderMap};
use axum_response::{Responder, Response};
use axum_validator::{Extension, Json};
use inject::AInjectProvider;

/// 控制器
pub struct LoginController;

impl LoginController {
    /// 登陆
    pub async fn login(
        Extension(provider): Extension<AInjectProvider>,
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
        headers: HeaderMap,
        Json(req): Json<LoginReq>,
    ) -> Responder<LoginResp> {
        let remote_addr = addr.ip().to_string();
        // Get the user agent from the request headers
        let user_agent = headers
            .get("User-Agent")
            .map_or("".to_owned(), |ua| ua.to_str().unwrap_or("").to_owned());
        let browser_info = BrowserInfo {
            remote_addr,
            user_agent,
        };

        let login_service: LoginService = provider.provide();
        let result = login_service.login(browser_info, req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }
}

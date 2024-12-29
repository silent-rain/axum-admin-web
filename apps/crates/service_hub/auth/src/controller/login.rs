//! 登陆

use crate::{
    dto::login::{BrowserInfo, LoginReq, LoginResp},
    LoginService,
};

use axum::{body::Body, extract::Request, Extension, Json};
use inject::AInjectProvider;
use response::{Responder, Response};

/// 控制器
pub struct LoginController;

impl LoginController {
    /// 登陆
    pub async fn login(
        Extension(provider): Extension<AInjectProvider>,
        request: Request<Body>,
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
        Json(req): Json<LoginReq>,
    ) -> Responder<LoginResp> {
        let remote_addr = addr
            .peer_addr()
            .map_or("".to_owned(), |addr| addr.ip().to_string());
        // Get the user agent from the request headers
        let user_agent = request
            .headers()
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

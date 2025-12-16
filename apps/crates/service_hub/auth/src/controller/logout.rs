//! 登出

use std::net::SocketAddr;

use axum::{extract::ConnectInfo, http::HeaderMap};
use axum_context::Context;
use axum_validator::Extension;
use tower_sessions::Session;

use axum_response::{Responder, Response};
use inject::AInjectProvider;

use crate::{dto::login::BrowserInfo, service::logout::Logoutervice};

/// 控制器
pub struct LogoutController;

impl LogoutController {
    /// 登出
    pub async fn logout(
        Extension(provider): Extension<AInjectProvider>,
        Extension(session): Extension<Session>,
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
        headers: HeaderMap,
        ctx: Context,
    ) -> Responder<()> {
        let remote_addr = addr.ip().to_string();
        // Get the user agent from the request headers
        let user_agent = headers
            .get("User-Agent")
            .map_or("".to_owned(), |ua| ua.to_str().unwrap_or("").to_owned());
        let browser_info = BrowserInfo {
            remote_addr,
            user_agent,
        };

        let login_service: Logoutervice = provider.provide();
        login_service.logout(ctx, browser_info, session).await?;

        Ok(Response::ok())
    }
}

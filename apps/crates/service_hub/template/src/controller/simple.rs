//! 简单示例

use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::{
    extract::{cookie::Cookie, PrivateCookieJar},
    headers::{ContentType, UserAgent},
    TypedHeader,
};

/// 控制器
pub struct SimpleController;

impl SimpleController {
    // go ahead and run "cargo run main.rs"
    // localhost:4000 should now print out your user agent
    pub async fn user_agent(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
        String::from(user_agent.as_str())
    }

    pub async fn user_agent_response() -> (TypedHeader<ContentType>, &'static str) {
        (TypedHeader(ContentType::text_utf8()), "Hello, World!")
    }

    pub async fn check_cookie(jar: PrivateCookieJar) -> impl IntoResponse {
        if jar.get("hello").is_none() {
            let _ = jar.clone().add(Cookie::new("hello", "world"));
        }
        let _ = jar.remove(Cookie::from("foo"));

        StatusCode::OK
    }
}

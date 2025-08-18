//! 简单示例

use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::{
    TypedHeader,
    extract::{PrivateCookieJar, cookie::Cookie},
    headers::{ContentType, UserAgent},
};

use axum_response::{Extension, Responder, Response};
use axum_validator::{Json, Query};
use inject::AInjectProvider;

use crate::dto::axum_validator::{SayHelloReq, SayHelloResp, SayHiReq, SayHiResp};

/// 控制器
pub struct SimpleController;

impl SimpleController {
    /// get sai hello
    /// http://127.0.0.1:3000/api/v1/template/axum-validators/say-hello?name=zhangsan
    pub async fn say_hello(
        Extension(_provider): Extension<AInjectProvider>,
        req: Query<SayHelloReq>,
    ) -> Responder<SayHelloResp> {
        let result = SayHelloResp {
            msg: format!("say hello: {}", req.name),
        };
        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// post sai hi
    pub async fn say_hi(req: Json<SayHiReq>) -> Responder<SayHiResp> {
        let result = SayHiResp {
            msg: format!("say hi: {}", req.name),
        };
        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

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

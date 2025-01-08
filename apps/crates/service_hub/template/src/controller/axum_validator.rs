//! 测试

use axum_response::{Extension, Responder, Response};
use axum_validator::{Json, Query};
use inject::AInjectProvider;

use crate::dto::axum_validator::{SayHelloReq, SayHelloResp, SayHiReq, SayHiResp};

/// 控制器
pub struct AxumValidatorController;

impl AxumValidatorController {
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
}

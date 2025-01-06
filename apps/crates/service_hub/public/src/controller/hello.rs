//! 测试

use axum_response::Responder;
use axum_response::Response;

use axum_validator::Json;
use axum_validator::Query;

use crate::dto::hello::HelloReq;
use crate::dto::hello::HelloResp;

/// 控制器
pub struct HelloController;

impl HelloController {
    /// hello
    pub async fn say_hello(req: Query<HelloReq>) -> Responder<HelloResp> {
        let result = HelloResp {
            data: format!("say hello: {}", req.name),
        };
        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// hi
    pub async fn say_hi(req: Json<HelloReq>) -> Responder<HelloResp> {
        let result = HelloResp {
            data: format!("say hi: {}", req.name),
        };
        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }
}

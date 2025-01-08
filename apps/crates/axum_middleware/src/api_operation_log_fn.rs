//! Api 操作日志中间件

use std::{net::SocketAddr, time::Instant};

use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use http_body_util::BodyExt;
use tracing::error;

use axum_context::Context;
use axum_response::ResponseErr;
use code::{Error, ErrorMsg};
use entity::log::log_api_operation;
use service_hub::{
    inject::AInjectProvider,
    log::{dto::api_operation::CreateApiOperationReq, ApiOperationService},
};

/// Api 操作日志中间件
/// ```
/// .layer(axum::middleware::from_fn(api_operation_log_middleware))
/// ```
pub async fn api_operation_log_middleware(
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, ResponseErr> {
    let mut api_log = ApiOperationLog::new(&request)
        .map_err(Into::<ResponseErr>::into)?
        .parse_req_info(&request);

    // 获取缓存的 req body bytes
    let (parts, body) = request.into_parts();

    let body_bytes = buffer_request_body(body).await?;

    // 创建请求体日志
    api_log = api_log
        .parse_req_body(body_bytes.clone())
        .create_api_operation_log()
        .await
        .map_err(Into::<ResponseErr>::into)?;

    // 重新构建请求
    let request = Request::from_parts(parts, Body::from(body_bytes));

    // 响应
    let resp = next.run(request).await;

    let status_code = resp.status();

    let (parts, body) = resp.into_parts();

    // 获取缓存的 resp body bytes
    let body_bytes = buffer_request_body(body).await?;

    // 创建响应体日志
    api_log
        .parse_resp_body(body_bytes.clone(), status_code)
        .create_api_operation_log()
        .await
        .map_err(Into::<ResponseErr>::into)?;

    let res = Response::from_parts(parts, Body::from(body_bytes));
    Ok(res)
}

/// the trick is to take the request apart, buffer the body, do what you need to do, then put
/// the request back together
async fn buffer_request_body<B>(body: B) -> Result<Bytes, ErrorMsg>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    // this won't work if the body is an long running stream
    let bytes = body
        .collect()
        .await
        .map_err(|err| Error::ParseRequestBodyError(err.to_string()).into_msg())?
        .to_bytes();
    Ok(bytes)
}

/// 操作日志处理
struct ApiOperationLog {
    inject_provider: AInjectProvider,
    data: Option<CreateApiOperationReq>,
    start_time: Instant,
}

impl ApiOperationLog {
    fn new(request: &Request) -> Result<Self, ErrorMsg> {
        // 请求开始的时间
        let start_time = std::time::Instant::now();

        // 全局依赖
        let inject_provider = match request.extensions().get::<AInjectProvider>() {
            Some(v) => v.clone(),
            None => {
                return Err(Error::InjectAproviderObj.into_msg());
            }
        };

        Ok(ApiOperationLog {
            inject_provider,
            data: None,
            start_time,
        })
    }

    /// 解析请求信息
    fn parse_req_info(mut self, request: &Request) -> Self {
        // 获取上下文
        let (user_id, username) = match request.extensions().get::<Context>() {
            Some(ctx) => (Some(ctx.get_user_id()), Some(ctx.get_user_name())),
            None => (None, None),
        };

        let status_code = StatusCode::OK.as_u16() as i32; // 默认请求成功
        let method = request.method().to_string();
        let path = request.uri().path().to_string();
        let query = request.uri().query().unwrap_or("").to_string();

        let content_type = request
            .headers()
            .get("Content-Type")
            .map_or("".to_string(), |v| {
                v.to_str().map_or("".to_string(), |v| v.to_string())
            })
            .to_uppercase();

        let request_id = request
            .headers()
            .get("x-request-id")
            .map_or("".to_string(), |v| {
                v.to_str().map_or("".to_string(), |v| v.to_string())
            });

        // 获取 remote_addr
        let remote_addr = request
            .extensions()
            .get::<SocketAddr>()
            .map(|socket_addr| socket_addr.ip().to_string())
            .unwrap_or("".to_string());

        // Get the user agent from the request headers
        let user_agent = request
            .headers()
            .get("User-Agent")
            .map_or("".to_owned(), |ua| ua.to_str().unwrap_or("").to_owned());

        self.data = Some(CreateApiOperationReq {
            user_id,
            username,
            request_id: Some(request_id),
            status_code,
            method,
            path,
            content_type,
            query: Some(query),
            body: None,
            remote_addr,
            user_agent,
            cost: 0,
            http_type: log_api_operation::enums::HttpType::Req,
            desc: None,
        });

        self
    }

    /// 解析请求体
    fn parse_req_body(mut self, req_body_bytes: Bytes) -> Self {
        let body = Self::body_bytes_to_string(&req_body_bytes)
            .map_or("body data parsing error ".to_string(), |v| v);
        let cost = self.start_time.elapsed().as_millis() as u64;

        let data = self.data.map(|mut data| {
            data.cost = cost;
            data.body = Some(body);

            data
        });
        self.data = data;

        self
    }

    /// 解析响应体
    fn parse_resp_body(mut self, req_body_bytes: Bytes, status_code: StatusCode) -> Self {
        let body = Self::body_bytes_to_string(&req_body_bytes)
            .map_or("body data parsing error ".to_string(), |v| v);
        let cost = self.start_time.elapsed().as_millis() as u64;

        let data = self.data.map(|mut data| {
            data.cost = cost;
            // TODO 添加字符限制, 如果太大则进行省略
            data.body = Some(body);
            data.http_type = log_api_operation::enums::HttpType::Rsp;

            // 图片body数据不入库
            if data.content_type != "multipart/form-data".to_uppercase() {
                data.body = None;
            }
            data.status_code = status_code.as_u16() as i32;

            data
        });
        self.data = data;

        self
    }

    /// 获取请求体的 body
    fn body_bytes_to_string(request_body: &Bytes) -> Result<String, ErrorMsg> {
        // 解析字符串为serde_json::Value
        let json_str = String::from_utf8_lossy(request_body).to_string();

        if json_str.is_empty() {
            return Ok("".to_owned());
        }

        let data: serde_json::Value = serde_json::from_str(&json_str).map_err(|err| {
            error!("body 数据转换错误, err: {err}");
            code::Error::JsonConvert(err.to_string())
                .into_msg()
                .with_msg("body 数据转换错误")
        })?;

        // 将Value转换为紧凑格式的字符串
        let body = serde_json::to_string(&data).map_err(|err| {
            error!("body 数据解析错误, err: {err}");
            code::Error::JsonConvert(err.to_string())
                .into_msg()
                .with_msg("body 数据解析错误")
        })?;

        Ok(body)
    }

    /// 创建操作日志
    async fn create_api_operation_log(self) -> Result<Self, code::ErrorMsg> {
        let data = match self.data {
            Some(ref v) => v,
            None => return Ok(self),
        };
        let api_operation_service: ApiOperationService = self.inject_provider.provide();
        let _user = api_operation_service.create(data.clone()).await?;

        Ok(self)
    }
}

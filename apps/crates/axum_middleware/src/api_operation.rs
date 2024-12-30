//! Api 操作日志中间件
use std::{boxed::Box, convert::Infallible, net::SocketAddr, task::Poll};

use code::{Error, ErrorMsg};
use entity::log::log_api_operation;
use response::ResponseErr;
use service_hub::{
    inject::AInjectProvider, log::dto::api_operation::CreateApiOperationReq,
    log::ApiOperationService,
};

use axum::{
    body::{Body, HttpBody},
    http::{Request, StatusCode},
    BoxError, Extension,
};
use axum_context::Context;
use bytes::Bytes;
use futures::future::BoxFuture;
use http_body_util::BodyExt;
use tower::{Layer, Service};
use tracing::error;

/// Api 操作日志中间件
#[derive(Clone)]
pub struct ApiOperationLayer;

impl<S> Layer<S> for ApiOperationLayer {
    type Service = ApiOperationMiddlewareService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ApiOperationMiddlewareService { inner }
    }
}

#[derive(Clone)]
pub struct ApiOperationMiddlewareService<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for ApiOperationMiddlewareService<S>
where
    S: Service<Request<ReqBody>, Response = axum::response::Response<ResBody>, Error = Infallible>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
    ReqBody: HttpBody<Data = Bytes> + Send + 'static + From<Body>,
    ReqBody::Error: std::fmt::Display,
    Infallible: From<<S as Service<Request<ReqBody>>>::Error>,
    ResBody: HttpBody<Data = Bytes> + Send + 'static + From<Body>,
    ResBody::Error: std::fmt::Display,
    ResBody::Error: Into<BoxError>,
    S::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = BoxError;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let start_time = std::time::Instant::now(); // 请求开始的时间

        let not_ready_inner = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, not_ready_inner);

        Box::pin(async move {
            // 全局依赖
            let inject_provider = match req.extensions().get::<Extension<AInjectProvider>>() {
                Some(v) => &v.0.clone(),
                None => {
                    return Err(Box::new(ResponseErr::new(Error::InjectAproviderObj)))
                        .map_err(Into::into)
                }
            };

            // 解析请求信息
            let mut data = Self::parse_req(&req);
            let content_type = req
                .headers()
                .get("Content-Type")
                .map_or("".to_string(), |v| {
                    v.to_str().map_or("".to_string(), |v| v.to_string())
                })
                .to_uppercase();

            // 获取请求体
            let (parts, body) = req.into_parts();
            let request_body_bytes = Self::body_buffer(body).await?;
            let req = Request::from_parts(parts, Body::from(request_body_bytes.clone()).into());

            // 添加请求操作日志
            data.cost = start_time.elapsed().as_millis() as u64;
            let body = Self::body_bytes_to_string(&request_body_bytes)
                .map_or("body data parsing error ".to_string(), |v| v);
            data.body = Some(body);

            // 将日志推入数据库
            if let Err(err) =
                Self::add_api_operation_log(inject_provider.clone(), data.clone()).await
            {
                return Err(Box::new(err)).map_err(Into::into);
            }

            // 响应
            let fut = inner.call(req).await?;
            let (parts, body) = fut.into_parts();
            let request_body_bytes = Self::body_buffer(body).await?;
            let body = Self::body_bytes_to_string(&request_body_bytes)
                .map_or("body data parsing error ".to_string(), |v| v);
            let res =
                axum::response::Response::from_parts(parts, Body::from(request_body_bytes).into());

            // 添加响应操作日志
            data.cost = start_time.elapsed().as_millis() as u64;
            data.http_type = log_api_operation::enums::HttpType::Rsp;
            // TODO 添加字符限制, 如果太大则进行省略
            data.body = Some(body);
            // 图片body数据不入库
            if content_type != "multipart/form-data".to_uppercase() {
                data.body = None;
            }
            data.status_code = res.status().as_u16() as i32;
            // 将日志推入数据库
            if let Err(err) = Self::add_api_operation_log(inject_provider.clone(), data).await {
                return Err(Box::new(err)).map_err(Into::into);
            }

            Ok(res)
        })
    }
}

impl<S> ApiOperationMiddlewareService<S> {
    async fn body_buffer<B>(body: B) -> Result<Bytes, ErrorMsg>
    where
        B: HttpBody<Data = Bytes>,
        B::Error: std::fmt::Display,
    {
        let bytes = match body.collect().await {
            Ok(collected) => collected.to_bytes(),
            Err(err) => {
                return Err(Error::InvalidParameter(err.to_string()).into_msg());
            }
        };

        Ok(bytes)
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
            code::Error::JsonConvert
                .into_msg()
                .with_msg("body 数据转换错误")
        })?;

        // 将Value转换为紧凑格式的字符串
        let body = serde_json::to_string(&data).map_err(|err| {
            error!("body 数据解析错误, err: {err}");
            code::Error::JsonConvert
                .into_msg()
                .with_msg("body 数据解析错误")
        })?;

        Ok(body)
    }

    /// 解析请求信息
    fn parse_req<ReqBody>(req: &Request<ReqBody>) -> CreateApiOperationReq {
        // 获取上下文
        let (user_id, username) = match req.extensions().get::<Context>() {
            Some(ctx) => (Some(ctx.get_user_id()), Some(ctx.get_user_name())),
            None => (None, None),
        };

        let status_code = StatusCode::OK.as_u16() as i32; // 默认请求成功
        let method = req.method().to_string();
        let path = req.uri().path().to_string();
        let query = req.uri().query().unwrap_or("").to_string();

        // 获取 remote_addr
        let remote_addr = req
            .extensions()
            .get::<SocketAddr>()
            .and_then(|socket_addr| Some(socket_addr.ip().to_string()))
            .unwrap_or("".to_string());

        // Get the user agent from the request headers
        let user_agent = req
            .headers()
            .get("User-Agent")
            .map_or("".to_owned(), |ua| ua.to_str().unwrap_or("").to_owned());

        CreateApiOperationReq {
            user_id,
            username,
            request_id: None,
            status_code,
            method,
            path,
            query: Some(query),
            body: None,
            remote_addr,
            user_agent,
            cost: 0,
            http_type: log_api_operation::enums::HttpType::Req,
            desc: None,
        }
    }

    /// 添加操作日志
    async fn add_api_operation_log(
        provider: AInjectProvider,
        data: CreateApiOperationReq,
    ) -> Result<(), code::ErrorMsg> {
        let api_operation_service: ApiOperationService = provider.provide();
        let _user = api_operation_service.create(data).await?;
        Ok(())
    }
}

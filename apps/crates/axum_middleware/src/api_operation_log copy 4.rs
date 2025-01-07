//! Api 操作日志中间件
use std::{boxed::Box, net::SocketAddr, task::Poll};

use axum::{
    body::{Body, HttpBody},
    extract::Request,
    BoxError,
};
use axum_context::Context;
use axum_extra::body::AsyncReadBody;
use bytes::Bytes;
use futures::future::BoxFuture;
use http_body_util::BodyExt;
use pin_project_lite::pin_project;
use tower::{Layer, Service};
use tracing::error;

use code::{Error, ErrorMsg};
use entity::log::log_api_operation;
use service_hub::{
    inject::AInjectProvider, log::dto::api_operation::CreateApiOperationReq,
    log::ApiOperationService,
};

use crate::error::create_error_response;

/// Api 操作日志中间件
#[derive(Clone)]
pub struct ApiOperationLogLayer;

impl<S> Layer<S> for ApiOperationLogLayer {
    type Service = ApiOperationLogMiddlewareService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ApiOperationLogMiddlewareService { inner }
    }
}

#[derive(Clone)]
pub struct ApiOperationLogMiddlewareService<S> {
    inner: S,
}

impl<S, ResBody> Service<Request> for ApiOperationLogMiddlewareService<S>
where
    S: Service<Request, Response = axum::response::Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Send + Sync + std::error::Error + Into<BoxError>,
    ResBody: HttpBody<Data = Bytes> + Send + 'static + From<Body>,
    ResBody::Error: Into<BoxError> + std::fmt::Display,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let not_ready_inner = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, not_ready_inner);

        Box::pin(async move {
            // 获取请求体
            let (parts, req_body) = req.into_parts();
            let request_body_bytes: Bytes = match Self::body_buffer(req_body).await {
                Ok(v) => v,
                Err(err) => return Ok(create_error_response(err)),
            };

            // 构建新的请求
            // TODOF 注意这里有问题
            // let req = Request::from_parts(parts, request_body_bytes.clone());
            let req = Request::from_parts(parts, Body::from(request_body_bytes.clone()));

            // 响应
            let fut = inner.call(req).await?;
            let (parts, resp_body) = fut.into_parts();
            let request_body_bytes = match Self::body_buffer(resp_body).await {
                Ok(v) => v,
                Err(err) => return Ok(create_error_response(err)),
            };

            let res =
                axum::response::Response::from_parts(parts, Body::from(request_body_bytes).into());

            Ok(res)
        })
    }
}

impl<S> ApiOperationLogMiddlewareService<S> {
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
}

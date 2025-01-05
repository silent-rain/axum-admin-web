//! 无错误返回的模板
use std::{convert::Infallible, task::Poll};

use axum::{
    body::HttpBody,
    http::{Request, Response},
    BoxError,
};
use bytes::Bytes;
use futures::future::BoxFuture;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct Template3Layer;

impl<S> Layer<S> for Template3Layer {
    type Service = TimeoutService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TimeoutService { inner }
    }
}

#[derive(Clone)]
pub struct TimeoutService<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for TimeoutService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>, Error = Infallible>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    Infallible: From<<S as Service<Request<ReqBody>>>::Error>,
    ResBody: HttpBody<Data = Bytes> + Default + Send + 'static,
    ResBody::Error: Into<BoxError>,
{
    type Response = Response<ResBody>;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let not_ready_inner = self.inner.clone();
        let mut ready_inner = std::mem::replace(&mut self.inner, not_ready_inner);

        Box::pin(async move {
            // todo
            ready_inner.call(req).await
        })
    }
}

//! 中间件模板
use std::{
    boxed::Box,
    convert::Infallible,
    task::{Context, Poll},
};

use axum::{body::HttpBody, http::Request, response::Response, BoxError};
use bytes::Bytes;
use futures::future::BoxFuture;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct TemplateLayer;

impl<S> Layer<S> for TemplateLayer {
    type Service = TemplateService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TemplateService { inner }
    }
}

#[derive(Clone)]
pub struct TemplateService<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for TemplateService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>, Error = Infallible>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    Infallible: From<<S as Service<Request<ReqBody>>>::Error>,
    ResBody: HttpBody<Data = Bytes> + Send + 'static,
    ResBody::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let not_ready_inner = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, not_ready_inner);

        Box::pin(async move {
            req.extensions_mut().insert(());
            inner.call(req).await
        })
    }
}

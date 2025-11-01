//! Context 中间件

use std::task::Poll;

use axum::{body::Body, extract::Request, http::Response};
use futures::future::BoxFuture;
use tower::{Layer, Service};

use crate::Context;

/// 上下文中间件
#[derive(Debug, Default, Clone)]
pub struct ContextLayer {}

impl ContextLayer {
    pub fn new() -> Self {
        ContextLayer {}
    }
}

impl<S> Layer<S> for ContextLayer {
    type Service = ContextService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ContextService { inner }
    }
}

#[derive(Clone)]
pub struct ContextService<S> {
    inner: S,
}

impl<S> Service<Request> for ContextService<S>
where
    S: Service<Request, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Send + Sync,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request) -> Self::Future {
        // Do something with `self.state`.
        let not_ready_inner = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, not_ready_inner);

        Box::pin(async move {
            // See `axum::RequestExt` for how to run extractors directly from  a `Request`.
            let context = Context {
                ..Default::default()
            };

            req.extensions_mut().insert(context);

            let resp = inner.call(req).await?;
            Ok(resp)
        })
    }
}

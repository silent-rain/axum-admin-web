//! 中间件
//! 初始化 Context

use crate::Context;

use axum::extract::Request;
use std::task::Poll;
use tower::{Layer, Service};

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

impl<S, B> Service<Request<B>> for ContextService<S>
where
    S: Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        // Do something with `self.state`.
        //
        // See `axum::RequestExt` for how to run extractors directly from
        // a `Request`.
        let context = Context {
            ..Default::default()
        };
        req.extensions_mut().insert(context);

        self.inner.call(req)
    }
}

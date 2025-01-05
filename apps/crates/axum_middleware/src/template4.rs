//! 自定义错误类型， 这是一个失败的示例
use axum::{
    body::HttpBody,
    http::{Request, Response},
    BoxError,
};
use axum_context::Context;
use bytes::Bytes;
use futures::future::BoxFuture;
use pin_project::pin_project;
use std::{convert::Infallible, future::Future, pin::Pin, task::Poll};
use tokio::time::Sleep;
use tower::{Layer, Service};

use response::ResponseErr;

#[derive(Clone)]
pub struct Template32Layer;

impl<S> Layer<S> for Template32Layer {
    type Service = TimeoutService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TimeoutService { inner }
    }
}

#[derive(Clone)]
pub struct TimeoutService<S> {
    inner: S,
}

impl<S, ReqBody> Service<Request<ReqBody>> for TimeoutService<S>
where
    S: Service<Request<ReqBody>>,
    S::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let response_future: <S as Service<Request<ReqBody>>>::Future = self.inner.call(req);

        ResponseFuture { response_future }
    }
}

#[pin_project]
pub struct ResponseFuture<F> {
    #[pin]
    response_future: F,
}

impl<F, Response, Error> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response, Error>>,
    Error: Into<BoxError>,
{
    type Output = Result<Response, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.response_future.poll(cx) {
            Poll::Ready(result) => {
                let result = result.map_err(Into::into);
                return Poll::Ready(result);
            }
            Poll::Pending => {}
        }

        // match this.sleep.poll(cx) {
        //     Poll::Ready(()) => {
        //         let error = Box::new(ResponseErr::new(code::Error::AuthIllegalRequest));
        //         return Poll::Ready(Err(error));
        //     }
        //     Poll::Pending => {}
        // }

        Poll::Pending
    }
}

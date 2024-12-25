use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use pin_project::pin_project;
use tokio::time::Sleep;
use tower::{BoxError, Layer, Service};

#[derive(Debug, Default, Clone)]
pub struct TimeoutLayer2 {
    timeout: Duration,
}

impl TimeoutLayer2 {
    pub fn new(timeout: Duration) -> Self {
        TimeoutLayer2 { timeout }
    }
}

impl<S> Layer<S> for TimeoutLayer2 {
    type Service = Timeout<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Timeout {
            inner,
            timeout: self.timeout,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Timeout<S> {
    inner: S,
    timeout: Duration,
}

impl<S, Request> Service<Request> for Timeout<S>
where
    S: Service<Request>,
    S::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let response_future = self.inner.call(request);
        let sleep = tokio::time::sleep(self.timeout);

        ResponseFuture {
            response_future,
            sleep,
        }
    }
}

#[pin_project]
pub struct ResponseFuture<F> {
    #[pin]
    response_future: F,
    #[pin]
    sleep: Sleep,
}

impl<F, Response, Error> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response, Error>>,
    Error: Into<BoxError>,
{
    type Output = Result<Response, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.response_future.poll(cx) {
            Poll::Ready(result) => {
                let result = result.map_err(Into::into);
                return Poll::Ready(result);
            }
            Poll::Pending => {}
        }

        match this.sleep.poll(cx) {
            Poll::Ready(()) => {
                let error = Box::new(TimeoutError());
                return Poll::Ready(Err(error));
            }
            Poll::Pending => {}
        }

        Poll::Pending
    }
}

#[derive(Debug, Default)]
struct TimeoutError();

impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("request timed out")
    }
}

impl std::error::Error for TimeoutError {}

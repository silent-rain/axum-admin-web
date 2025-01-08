//! axum 原生 Request<Body>
//! 不兼容 RequestBodyLimitLayer 这类子自定义 Body 类型
use std::{sync::Arc, task::Poll};

use axum::{body::Body, extract::Request, http::Response};
use futures::future::BoxFuture;
use tower::{BoxError, Layer, Service};
use tracing::error;

use app_state::AppState;
use axum_context::{ApiAuthType, Context};
use code::Error;
use service_hub::inject::AInjectProvider;

use crate::error::create_error_response;

#[derive(Clone)]
pub struct Template5Layer;

impl<S> Layer<S> for Template5Layer {
    type Service = TimeoutService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TimeoutService { inner }
    }
}

#[derive(Clone)]
pub struct TimeoutService<S> {
    inner: S,
}

impl<S> Service<Request> for TimeoutService<S>
where
    S: Service<Request> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<BoxError>,
    S::Response: Into<Response<Body>>,
{
    type Response = Response<Body>;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request) -> Self::Future {
        let not_ready_inner = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, not_ready_inner);

        Box::pin(async move {
            match req.extensions().get::<Arc<AppState>>() {
                Some(_v) => (),
                None => {
                    error!("get app state failed");
                }
            };

            let _inject_provider = match req.extensions().get::<AInjectProvider>() {
                Some(v) => v,
                None => {
                    let resp = create_error_response(Error::InjectAproviderObj.into_msg());
                    return Ok(resp);
                }
            };

            // ...

            if let Some(ctx) = req.extensions_mut().get_mut::<Context>() {
                ctx.set_user_id(1);
                ctx.set_user_name("demo".to_owned());
                ctx.set_api_auth_type(ApiAuthType::Openapi);
            }

            // ...

            let resp = inner.call(req).await?.into();
            Ok(resp)
        })
    }
}

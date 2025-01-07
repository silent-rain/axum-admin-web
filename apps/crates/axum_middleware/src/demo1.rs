//! middleware Demo1
use std::{boxed::Box, sync::Arc, task::Poll};

use app_state::AppState;
use axum::{
    body::{Body, HttpBody},
    extract::Request,
    BoxError,
};
use axum_context::{ApiAuthType, Context};
use bytes::Bytes;
use futures::future::BoxFuture;
use tower::{Layer, Service};

use code::Error; // Custom error
use service_hub::inject::AInjectProvider;
use tracing::error;

use crate::error::create_error_response;

#[derive(Clone)]
pub struct Demo1Layer;

impl<S> Layer<S> for Demo1Layer {
    type Service = DemoService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        DemoService { inner }
    }
}

#[derive(Clone)]
pub struct DemoService<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for DemoService<S>
where
    S: Service<Request<ReqBody>, Response = axum::http::Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Send + Sync + std::error::Error + Into<BoxError>,
    ReqBody: Send + 'static,
    ResBody: HttpBody<Data = Bytes> + Send + 'static + From<Body>,
    ResBody::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
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

            let resp = inner.call(req).await?;
            Ok(resp)
        })
    }
}

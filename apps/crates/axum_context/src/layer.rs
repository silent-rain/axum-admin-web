//! 中间件
//! 初始化 Context

use crate::{error::create_error_response, Context};

use axum::{body::Body, extract::Request, http::Response};
use code::Error;
use futures::future::BoxFuture;
use std::task::Poll;
use tower::{Layer, Service};
use tower_sessions::{session::Id, Session};
use tracing::error;

const HEADER_SESSION_ID: &str = "x-session-id";

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
            let (session, header_session_id) = match Self::get_session_id(&req) {
                Ok(v) => v,
                Err(err) => {
                    return Ok(create_error_response(err));
                }
            };

            // 添加 session_id
            let mut session_id = header_session_id;
            if session_id.is_empty() {
                error!("headers not session id");
                let new_session_id = Id::default();
                // session
                //     .insert(new_session_id.to_string().as_str(), true)
                //     .await
                //     .map_err(|err| Error::SessionIdInsertError(err.to_string()))?;
                match session
                    .insert(new_session_id.to_string().as_str(), true)
                    .await
                    .map_err(|err| Error::SessionIdInsertError(err.to_string()))
                {
                    Ok(v) => v,
                    Err(err) => return Ok(create_error_response(err.into())),
                };

                session_id = new_session_id.to_string();
            }

            // See `axum::RequestExt` for how to run extractors directly from  a `Request`.
            let context = Context {
                session_id: session_id,
                ..Default::default()
            };

            req.extensions_mut().insert(context);

            let resp = inner.call(req).await?;
            Ok(resp)
        })
    }
}

impl<S> ContextService<S> {
    /// 获取 session id
    fn get_session_id(req: &Request) -> Result<(&Session, String), code::ErrorMsg> {
        let header_session_id = req
            .headers()
            .get(HEADER_SESSION_ID)
            .map_or("", |v| v.to_str().map_or("", |v| v));

        // Session
        let session = req.extensions().get::<Session>().ok_or_else(|| {
            Error::SessionIdNotFound
                .into_msg()
                .with_msg("session extension  error")
        })?;

        Ok((session, header_session_id.to_string()))
    }
}

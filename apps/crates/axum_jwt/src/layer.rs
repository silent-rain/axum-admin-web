//! JWT权限中间件
use std::{boxed::Box, task::Poll};

use axum::{body::Body, extract::Request, http::Response};
use axum_context::{ApiAuthType, Context};
use futures::future::BoxFuture;
use tower::{Layer, Service};
use tracing::error;

use crate::{Claims, Error};

/// JWT权限中间件
#[derive(Clone)]
pub struct JwtLayer {
    authorization: String,
    authorization_bearer: String,
    auth_white_list: Vec<String>,
}

impl Default for JwtLayer {
    fn default() -> Self {
        JwtLayer {
            authorization: "Authorization".to_string(),
            authorization_bearer: "Bearer ".to_string(),
            auth_white_list: vec![],
        }
    }
}

impl JwtLayer {
    pub fn with_auth_white_list(mut self, auth_white_list: Vec<String>) -> Self {
        self.auth_white_list = auth_white_list;
        self
    }

    pub fn with_authorization(mut self, authorization: String) -> Self {
        self.authorization = authorization;
        self
    }

    pub fn with_authorization_bearer(mut self, authorization_bearer: String) -> Self {
        self.authorization_bearer = authorization_bearer;
        self
    }
}

impl<S> Layer<S> for JwtLayer {
    type Service = JwtService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        JwtService {
            inner,
            authorization: self.authorization.clone(),
            authorization_bearer: self.authorization_bearer.clone(),
            auth_white_list: self.auth_white_list.clone(),
        }
    }
}

#[derive(Clone)]
pub struct JwtService<S> {
    inner: S,
    authorization: String,
    authorization_bearer: String,
    auth_white_list: Vec<String>,
}

impl<S> Service<Request> for JwtService<S>
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
        let authorization = self.authorization.clone();
        let authorization_bearer = self.authorization_bearer.clone();
        let auth_white_list = self.auth_white_list.clone();
        let not_ready_inner = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, not_ready_inner);

        Box::pin(async move {
            // 不存在系统鉴权标识时, 则直接通过
            // 本地仅有JTW鉴权时, 则无需判断
            // if req.headers().get(authorization.clone()).is_none() {
            //     let resp = inner.call(req).await?;
            //     return Ok(resp);
            // }

            // 白名单放行
            let path = req.uri().path();
            if auth_white_list.clone().contains(&path.to_string()) {
                let resp = inner.call(req).await?;
                return Ok(resp);
            }

            // 获取系统鉴权标识Token
            let system_token =
                match Self::get_system_api_token(&req, authorization, authorization_bearer) {
                    Ok(v) => v,
                    Err(err) => {
                        return Ok(err.into());
                    }
                };

            // 解析系统接口Token
            let claims = match Self::parse_system_token(system_token.clone()) {
                Ok(v) => v,
                Err(err) => {
                    error!("检查系统鉴权异常, err: {err:?}");
                    return Ok(err.into());
                }
            };

            // 设置上下文
            if let Some(ctx) = req.extensions_mut().get_mut::<Context>() {
                ctx.set_user_id(claims.user_id);
                ctx.set_user_name(claims.username);
                ctx.set_api_auth_type(ApiAuthType::System);
            }

            // 响应
            let resp = inner.call(req).await?;
            Ok(resp)
        })
    }
}

impl<S> JwtService<S> {
    /// 获取系统接口鉴权Token
    fn get_system_api_token<ReqBody>(
        req: &Request<ReqBody>,
        authorization: String,
        authorization_bearer: String,
    ) -> Result<String, Error> {
        let authorization = req
            .headers()
            .get(authorization.clone())
            .map_or("", |v| v.to_str().map_or("", |v| v));

        if authorization.is_empty() {
            error!("鉴权标识为空, 请求头未包含 Authorization 字段");
            return Err(Error::HeadersNotAuthorization);
        }
        if !authorization.starts_with(&authorization_bearer) {
            error!(
                "请求头 Authorization 字段缺失 Bearer 前缀, 非法请求, authorization: {authorization}"
            );
            return Err(Error::HeadersNotAuthorizationBearer);
        }

        let token = authorization.replace(&authorization_bearer, "");

        Ok(token)
    }

    /// 解析系统接口Token
    fn parse_system_token(token: String) -> Result<Claims, Error> {
        // 解码 Token
        let claims = Claims::decode_token(&token)?;
        // 验证 Token
        claims.verify()?;

        Ok(claims)
    }
}

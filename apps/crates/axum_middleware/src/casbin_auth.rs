//! Casbin权限中间件
//! RBAC 鉴权
use std::{boxed::Box, task::Poll};

use axum::extract::Request;
use axum_context::Context;
use casbin::{
    prelude::{DefaultModel, Enforcer, MemoryAdapter},
    CoreApi, MgmtApi,
};
use futures::future::BoxFuture;
use tower::{Layer, Service};
use tracing::{error, info};

use code::{Error, ErrorMsg};
use service_hub::permission::OpenapiService;
use service_hub::user::UserRoleRelService;
use service_hub::{inject::AInjectProvider, user::cached::UserCached};

use crate::{constant::AUTH_WHITE_LIST, error::create_error_response};

const MODEL: &str = "
[request_definition]
r = sub, obj, act

[policy_definition]
p = sub, obj, act

[role_definition]
g = _, _

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = g(r.sub, p.sub) && (r.obj == p.obj) && (r.act == p.act)
";

const _POLICY: &str = "
p, alice, /users, GET
p, bob, /users/1/status, PUT
g, alice, admin
";

/// Casbin权限中间件
#[derive(Clone)]
pub struct CasbinAuthLayer;

impl<S> Layer<S> for CasbinAuthLayer {
    type Service = CasbinAuthService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CasbinAuthService { inner }
    }
}

#[derive(Clone)]
pub struct CasbinAuthService<S> {
    inner: S,
}

impl<S> Service<Request> for CasbinAuthService<S>
where
    S: Service<Request, Response = axum::response::Response> + Clone + Send + 'static,
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

    fn call(&mut self, req: Request) -> Self::Future {
        let not_ready_inner = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, not_ready_inner);

        Box::pin(async move {
            // 白名单放行
            let path = req.uri().path();
            if AUTH_WHITE_LIST.contains(&path) {
                let resp = inner.call(req).await?;
                return Ok(resp);
            }

            let method = req.method().as_str();

            // 获取用户ID
            let user_id = match req.extensions().get::<Context>() {
                Some(ctx) => ctx.get_user_id(),
                None => {
                    let resp = inner.call(req).await?;
                    return Ok(resp);
                }
            };

            // 全局依赖
            let inject_provider = match req.extensions().get::<AInjectProvider>() {
                Some(v) => v.clone(),
                None => {
                    return Ok(create_error_response(Error::InjectAproviderObj.into_msg()));
                }
            };

            // Casbin
            let mut cb = match Casbin::new(inject_provider).await {
                Ok(v) => v,
                Err(err) => return Ok(create_error_response(err)),
            };
            // 执行
            match cb
                .enforce(user_id, method.to_string(), path.to_string())
                .await
            {
                Ok(v) => v,
                Err(err) => return Ok(create_error_response(err)),
            };

            // 响应
            let resp = inner.call(req).await?;
            Ok(resp)
        })
    }
}

struct Casbin {
    inject_provider: AInjectProvider,
    enforcer: Enforcer,
}

impl Casbin {
    async fn new(inject_provider: AInjectProvider) -> Result<Self, ErrorMsg> {
        // 加载模型
        let m = DefaultModel::from_str(MODEL)
            .await
            .map_err(|err| Error::CasbinError(err))?;
        // 加载策略
        let adapter = MemoryAdapter::default();
        // 创建 Enforcer
        let enforcer = Enforcer::new(m, adapter)
            .await
            .map_err(|err| Error::CasbinError(err))?;

        Ok(Casbin {
            inject_provider: inject_provider.clone(),
            enforcer,
        })
    }

    /// 获取角色权限列表
    async fn get_p_policies(&self) -> Result<Vec<Vec<String>>, ErrorMsg> {
        // 获取接口角色关系列表
        let openapi_service: OpenapiService = self.inject_provider.provide();
        let role_openapi_permissions = openapi_service.role_openapi_permissions().await?;

        // ["admin", "/users/1/status", "PUT"]
        let p_policies: Vec<Vec<String>> = role_openapi_permissions
            .iter()
            .map(|v| vec![v.role_id.to_string(), v.path.clone(), v.method.clone()])
            .collect();

        Ok(p_policies)
    }

    /// 获取角色组权限
    async fn get_g_policies(&self) -> Result<Vec<Vec<String>>, ErrorMsg> {
        // 获取用户角色关系列表
        let user_role_rel_service: UserRoleRelService = self.inject_provider.provide();
        let (user_role_rels, _) = user_role_rel_service.all().await?;

        // ["alice", "admin"]
        let g_policies: Vec<Vec<String>> = user_role_rels
            .iter()
            .map(|v| vec![v.user_id.to_string(), v.role_id.to_string()])
            .collect();

        Ok(g_policies)
    }

    /// 执行
    async fn enforce(
        &mut self,
        user_id: i32,
        path: String,
        method: String,
    ) -> Result<bool, ErrorMsg> {
        // 获取缓存
        if let Ok(permission) =
            UserCached::get_user_openapi_access_permission(user_id, path.clone(), method.clone())
                .await
        {
            if permission {
                info!(
                    "openapi access permission, cached, user_id: {user_id}, method: {method}, path: {path}"
                );
                return Ok(true);
            }
        }

        let p_policies = self.get_p_policies().await?;
        let g_policies = self.get_g_policies().await?;

        // 添加策略
        self.enforcer
            .add_policies(p_policies)
            .await
            .map_err(|err| Error::CasbinError(err))?;
        // 添加角色
        self.enforcer
            .add_grouping_policies(g_policies)
            .await
            .map_err(|err| Error::CasbinError(err))?;

        // 执行权限检查
        // ("alice", "/users", "GET")
        let result = self
            .enforcer
            .enforce((user_id.clone(), path.clone(), method.clone()))
            .map_err(|err| Error::CasbinError(err))?;

        // 权限判断
        if !result {
            error!("{user_id} {method} {path}, No access permission");
            return Err(Error::CasbinNoAccessPermission.into_msg());
        }

        // 设置缓存
        UserCached::set_user_openapi_access_permission(
            user_id.clone(),
            path.clone(),
            method.clone(),
        )
        .await;

        info!("openapi access permission, user_id: {user_id}, method: {method}, path: {path}");

        Ok(false)
    }
}

#[cfg(test)]
mod test {
    use casbin::MgmtApi;

    use super::*;

    #[tokio::test]
    async fn test_csabin() -> Result<(), Error> {
        // 加载模型
        let m = DefaultModel::from_str(MODEL).await?;
        // 加载策略
        let adapter = MemoryAdapter::default();

        // 创建 Enforcer
        let mut e = Enforcer::new(m, adapter).await?;
        e.add_policy(
            ["admin", "/users", "GET"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        )
        .await?;
        e.add_policies(vec![["admin2", "/users/1/status", "PUT"]
            .iter()
            .map(|s| s.to_string())
            .collect()])
            .await?;

        // 添加角色赋值
        e.add_grouping_policies(vec![["alice", "admin"]
            .iter()
            .map(|s| s.to_string())
            .collect()])
            .await?;

        // 检查是否存在策略
        assert!(!e.has_policy(
            ["alice", "/users/1/status", "PUT"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        ));

        // 访问权限
        assert!(e.enforce(("alice", "/users", "GET"))?);
        assert!(!e.enforce(("alice", "/users", "PUT"))?);
        assert!(!e.enforce(("alice1", "/users", "PUT"))?);

        Ok(())
    }
}

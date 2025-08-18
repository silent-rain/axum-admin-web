//! 用户角色关系管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::user_role_rel::{
        BatchCreateUserRoleRelReq, BatchCreateUserRoleRelResp, BatchDeleteUserRoleRelReq,
        BatchDeleteUserRoleRelResp, GetUserRoleRelsReq, GetUserRoleRelsResp,
    },
    service::user_role_rel::UserRoleRelService,
};

/// 控制器
pub struct UserRoleRelController;

impl UserRoleRelController {
    /// 获取用户角色关系列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetUserRoleRelsReq>,
    ) -> Responder<GetUserRoleRelsResp> {
        let user_role_rel_service: UserRoleRelService = provider.provide();
        let (results, total) = user_role_rel_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 批量创建用户角色关系
    pub async fn batch_create(
        Extension(provider): Extension<AInjectProvider>,
        Json(data): Json<BatchCreateUserRoleRelReq>,
    ) -> Responder<BatchCreateUserRoleRelResp> {
        let user_role_rel_service: UserRoleRelService = provider.provide();
        let _result = user_role_rel_service.batch_create(data).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 批量删除用户角色关系
    pub async fn batch_delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(data): Json<BatchDeleteUserRoleRelReq>,
    ) -> Responder<BatchDeleteUserRoleRelResp> {
        let user_role_rel_service: UserRoleRelService = provider.provide();
        let _result = user_role_rel_service.batch_delete(data.ids.clone()).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}

//! 菜单角色关系管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::menu_role_rel::{
        BatchCreateMenuRoleRelReq, BatchDeleteMenuRoleRelReq, GetMenuRoleRelsReq,
        GetMenuRoleRelsResp,
    },
    service::menu_role_rel::MenuRoleRelService,
};

/// 控制器
pub struct MenuRoleRelController;

impl MenuRoleRelController {
    /// 获取菜单角色关系列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetMenuRoleRelsReq>,
    ) -> Responder<GetMenuRoleRelsResp> {
        let menu_role_rel_service: MenuRoleRelService = provider.provide();
        let (results, total) = menu_role_rel_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 批量创建菜单角色关系
    pub async fn batch_create(
        Extension(provider): Extension<AInjectProvider>,
        Json(data): Json<BatchCreateMenuRoleRelReq>,
    ) -> Responder<()> {
        let menu_role_rel_service: MenuRoleRelService = provider.provide();
        let _result = menu_role_rel_service.batch_create(data).await?;

        Ok(Response::ok())
    }

    /// 批量删除菜单角色关系
    pub async fn batch_delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(data): Json<BatchDeleteMenuRoleRelReq>,
    ) -> Responder<()> {
        let menu_role_rel_service: MenuRoleRelService = provider.provide();
        let _result = menu_role_rel_service.batch_delete(data.ids.clone()).await?;

        Ok(Response::ok())
    }
}

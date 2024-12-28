//! 菜单角色关系管理

use crate::{
    dto::menu_role_rel::{
        BatchCreateMenuRoleRelReq, BatchCreateMenuRoleRelResp, BatchDeleteMenuRoleRelReq,
        BatchDeleteMenuRoleRelResp, GetMenuRoleRelsReq, GetMenuRoleRelsResp,
    },
    service::menu_role_rel::MenuRoleRelService,
};

use axum::{extract::Query, Extension, Json};
use inject::AInjectProvider;
use response::{Responder, Response};

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

        let resp = Response::data_list(results, total).to_json::<GetMenuRoleRelsResp>()?;
        Ok(resp)
    }

    /// 批量创建菜单角色关系
    pub async fn batch_create(
        Extension(provider): Extension<AInjectProvider>,
        Json(data): Json<BatchCreateMenuRoleRelReq>,
    ) -> Responder<BatchCreateMenuRoleRelResp> {
        let menu_role_rel_service: MenuRoleRelService = provider.provide();
        let _result = menu_role_rel_service.batch_create(data).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 批量删除菜单角色关系
    pub async fn batch_delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(data): Json<BatchDeleteMenuRoleRelReq>,
    ) -> Responder<BatchDeleteMenuRoleRelResp> {
        let menu_role_rel_service: MenuRoleRelService = provider.provide();
        let _result = menu_role_rel_service.batch_delete(data.ids.clone()).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}

//! 角色管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::role::{
        CreateRoleReq, CreateRoleResp, DeleteRoleReq, DeleteRoleResp, GetRoleReq, GetRoleResp,
        GetRolesReq, GetRolesResp, UpdateRoleReq, UpdateRoleResp, UpdateRoleStatusReq,
        UpdateRoleStatusResp,
    },
    service::role::RoleService,
};

/// 控制器
pub struct RoleController;

impl RoleController {
    /// 获取角色列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetRolesReq>,
    ) -> Responder<GetRolesResp> {
        let role_service: RoleService = provider.provide();
        let (results, total) = role_service.list(req).await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取角色信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetRoleReq>,
    ) -> Responder<GetRoleResp> {
        let role_service: RoleService = provider.provide();
        let result = role_service.info(req).await?;

        let resp = Response::data(result.into());
        Ok(resp)
    }

    /// 添加角色
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(data): Json<CreateRoleReq>,
    ) -> Responder<CreateRoleResp> {
        let role_service: RoleService = provider.provide();
        let _result = role_service.create(data).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 更新角色
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateRoleReq>,
    ) -> Responder<UpdateRoleResp> {
        let role_service: RoleService = provider.provide();
        let _result = role_service.update(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 更新角色状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateRoleStatusReq>,
    ) -> Responder<UpdateRoleStatusResp> {
        let role_service: RoleService = provider.provide();
        role_service.update_status(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 删除角色
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteRoleReq>,
    ) -> Responder<DeleteRoleResp> {
        let role_service: RoleService = provider.provide();
        let _result = role_service.delete(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }
}

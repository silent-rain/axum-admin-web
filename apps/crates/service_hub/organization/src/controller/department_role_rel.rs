//! 部门角色关系管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};
use inject::AInjectProvider;

use crate::{
    dto::department_role_rel::{
        BatchCreateDepartmentRoleRelReq, BatchDeleteDepartmentRoleRelReq, GetDepartmentRoleRelsReq,
        GetDepartmentRoleRelsResp,
    },
    service::department_role_rel::DepartmentRoleRelService,
};

/// 控制器
pub struct DepartmentRoleRelController;

impl DepartmentRoleRelController {
    /// 获取部门角色关系列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetDepartmentRoleRelsReq>,
    ) -> Responder<GetDepartmentRoleRelsResp> {
        let department_role_rel_service: DepartmentRoleRelService = provider.provide();
        let (results, total) = department_role_rel_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 批量创建部门角色关系
    pub async fn batch_create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<BatchCreateDepartmentRoleRelReq>,
    ) -> Responder<()> {
        let department_role_rel_service: DepartmentRoleRelService = provider.provide();
        let _result = department_role_rel_service.batch_create(req).await?;

        Ok(Response::ok())
    }

    /// 批量删除部门角色关系
    pub async fn batch_delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<BatchDeleteDepartmentRoleRelReq>,
    ) -> Responder<()> {
        let department_role_rel_service: DepartmentRoleRelService = provider.provide();
        let _result = department_role_rel_service
            .batch_delete(req.ids.clone())
            .await?;

        Ok(Response::ok())
    }
}

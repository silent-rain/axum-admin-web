//! 部门管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};
use inject::AInjectProvider;

use crate::{
    dto::department::{
        CreateDepartmentReq, CreateDepartmentResp, DeleteDepartmentReq, DeleteDepartmentResp,
        GetDepartmentReq, GetDepartmentResp, GetDepartmentTreeReq, GetDepartmentTreeResp,
        GetDepartmentsReq, GetDepartmentsResp, UpdateDepartmentReq, UpdateDepartmentResp,
        UpdateDepartmentStatusReq, UpdateDepartmentStatusResp,
    },
    service::department::DepartmentService,
};
/// 控制器
pub struct DepartmentController;

impl DepartmentController {
    /// 获取部门列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetDepartmentsReq>,
    ) -> Responder<GetDepartmentsResp> {
        let department_service: DepartmentService = provider.provide();
        let (results, total) = department_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取部门树列表
    pub async fn tree(
        Extension(provider): Extension<AInjectProvider>,
        Query(_req): Query<GetDepartmentTreeReq>,
    ) -> Responder<GetDepartmentTreeResp> {
        let department_service: DepartmentService = provider.provide();
        let result = department_service.tree().await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 获取部门信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetDepartmentReq>,
    ) -> Responder<GetDepartmentResp> {
        let department_service: DepartmentService = provider.provide();
        let result = department_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加部门
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateDepartmentReq>,
    ) -> Responder<CreateDepartmentResp> {
        let department_service: DepartmentService = provider.provide();
        let _result = department_service.create(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新部门
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDepartmentReq>,
    ) -> Responder<UpdateDepartmentResp> {
        let department_service: DepartmentService = provider.provide();
        let _result = department_service.update(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新部门状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDepartmentStatusReq>,
    ) -> Responder<UpdateDepartmentStatusResp> {
        let department_service: DepartmentService = provider.provide();
        department_service.update_status(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 删除部门
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteDepartmentReq>,
    ) -> Responder<DeleteDepartmentResp> {
        let department_service: DepartmentService = provider.provide();
        let _result = department_service.delete(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}

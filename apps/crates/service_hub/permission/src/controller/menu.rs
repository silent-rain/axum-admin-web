//! 菜单管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::menu::{
        CreateMenuReq, DeleteMenuReq, GetMenuChildrenReq, GetMenuChildrenResp, GetMenuReq,
        GetMenuResp, GetMenuTreeReq, GetMenuTreeResp, GetMenusReq, GetMenusResp, UpdateMenuReq,
        UpdateMenuStatusReq,
    },
    service::menu::MenuService,
};

/// 控制器
pub struct MenuController;

impl MenuController {
    /// 获取菜单列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetMenusReq>,
    ) -> Responder<GetMenusResp> {
        let menu_service: MenuService = provider.provide();
        let (results, total) = menu_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取菜单树列表
    pub async fn tree(
        Extension(provider): Extension<AInjectProvider>,
        Query(_req): Query<GetMenuTreeReq>,
    ) -> Responder<GetMenuTreeResp> {
        let menu_service: MenuService = provider.provide();
        let result = menu_service.tree().await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 获取子菜单树列表
    pub async fn children(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetMenuChildrenReq>,
    ) -> Responder<GetMenuChildrenResp> {
        let menu_service: MenuService = provider.provide();
        let (results, total) = menu_service.children(req.pid).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取菜单信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetMenuReq>,
    ) -> Responder<GetMenuResp> {
        let menu_service: MenuService = provider.provide();
        let result = menu_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加菜单
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateMenuReq>,
    ) -> Responder<()> {
        let menu_service: MenuService = provider.provide();
        let _result = menu_service.create(req).await?;

        Ok(Response::ok())
    }

    /// 更新菜单
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateMenuReq>,
    ) -> Responder<()> {
        let menu_service: MenuService = provider.provide();
        let _result = menu_service.update(req).await?;

        Ok(Response::ok())
    }

    /// 更新菜单状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateMenuStatusReq>,
    ) -> Responder<()> {
        let menu_service: MenuService = provider.provide();
        menu_service.update_status(req).await?;

        Ok(Response::ok())
    }

    /// 删除菜单
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteMenuReq>,
    ) -> Responder<()> {
        let menu_service: MenuService = provider.provide();
        let _result = menu_service.delete(req).await?;

        Ok(Response::ok())
    }
}

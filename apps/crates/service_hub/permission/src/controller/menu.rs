//! 菜单管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::menu::{
        CreateMenuReq, CreateMenuResp, DeleteMenuReq, DeleteMenuResp, GetMenuChildrenReq,
        GetMenuChildrenResp, GetMenuReq, GetMenuResp, GetMenuTreeReq, GetMenuTreeResp, GetMenusReq,
        GetMenusResp, UpdateMenuReq, UpdateMenuResp, UpdateMenuStatusReq, UpdateMenuStatusResp,
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

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取菜单树列表
    pub async fn tree(
        Extension(provider): Extension<AInjectProvider>,
        Query(_req): Query<GetMenuTreeReq>,
    ) -> Responder<GetMenuTreeResp> {
        let menu_service: MenuService = provider.provide();
        let result = menu_service.tree().await?;

        let resp = Response::data(result.into());
        Ok(resp)
    }

    /// 获取子菜单树列表
    pub async fn children(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetMenuChildrenReq>,
    ) -> Responder<GetMenuChildrenResp> {
        let menu_service: MenuService = provider.provide();
        let (results, total) = menu_service.children(req.pid).await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取菜单信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetMenuReq>,
    ) -> Responder<GetMenuResp> {
        let menu_service: MenuService = provider.provide();
        let result = menu_service.info(req).await?;

        let resp = Response::data(result.into());
        Ok(resp)
    }

    /// 添加菜单
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateMenuReq>,
    ) -> Responder<CreateMenuResp> {
        let menu_service: MenuService = provider.provide();
        let _result = menu_service.create(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 更新菜单
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateMenuReq>,
    ) -> Responder<UpdateMenuResp> {
        let menu_service: MenuService = provider.provide();
        let _result = menu_service.update(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 更新菜单状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateMenuStatusReq>,
    ) -> Responder<UpdateMenuStatusResp> {
        let menu_service: MenuService = provider.provide();
        menu_service.update_status(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 删除菜单
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteMenuReq>,
    ) -> Responder<DeleteMenuResp> {
        let menu_service: MenuService = provider.provide();
        let _result = menu_service.delete(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }
}

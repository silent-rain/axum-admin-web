//! 菜单管理

use log::error;
use nject::injectable;
use sea_orm::Set;

use err_code::{Error, ErrorMsg};
use database::utils::GenericTree;

use crate::{
    dao::menu::MenuDao,
    dto::menu::{
        CreateMenuReq, DeleteMenuReq, GetMenuReq, GetMenusReq, UpdateMenuReq, UpdateMenuStatusReq,
    },
    entity::menu,
};

/// 服务层
#[injectable]
pub struct MenuService {
    menu_dao: MenuDao,
}

impl MenuService {
    /// 获取列表数据
    pub async fn list(&self, req: GetMenusReq) -> Result<(Vec<menu::Model>, u64), ErrorMsg> {
        // 获取所有数据
        if let Some(true) = req.all {
            return self.menu_dao.all().await.map_err(|err| {
                error!("查询所有菜单失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询所有菜单失败")
            });
        }

        let (results, total) = self.menu_dao.list(req).await.map_err(|err| {
            error!("查询菜单列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询菜单列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取树列表数据
    pub async fn tree(&self) -> Result<Vec<GenericTree<menu::Model>>, ErrorMsg> {
        let (results, _total) = self.menu_dao.all().await.map_err(|err| {
            error!("查询菜单列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询菜单列表失败")
        })?;

        // 将列表转换为树列表
        let results = GenericTree::to_tree(&results, None);
        Ok(results)
    }

    /// 获取子列表数据
    pub async fn children(&self, pid: i32) -> Result<(Vec<menu::Model>, u64), ErrorMsg> {
        let results = self.menu_dao.children(pid).await.map_err(|err| {
            error!("查询子菜单列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询子菜单列表失败")
        })?;
        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, req: GetMenuReq) -> Result<menu::Model, ErrorMsg> {
        let result = self
            .menu_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询菜单信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询菜单信息失败")
            })?
            .ok_or_else(|| {
                error!("菜单不存在");
                Error::DbQueryEmptyError.into_err_with_msg("菜单不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn create(&self, req: CreateMenuReq) -> Result<menu::Model, ErrorMsg> {
        let model = menu::ActiveModel {
            pid: Set(req.pid),
            title: Set(req.title),
            icon_class: Set(req.icon_class),
            menu_type: Set(req.menu_type as i8),
            open_method: Set(req.open_method as i8),
            path: Set(req.path),
            component_path: Set(req.component_path),
            redirect_to: Set(req.redirect_to),
            link: Set(req.link),
            link_target: Set(req.link_target.map(|v| v.into())),
            is_hidden: Set(req.is_hidden),
            is_always_show_root: Set(req.is_always_show_root),
            permission: Set(req.permission),
            sort: Set(req.sort),
            desc: Set(req.desc),
            status: Set(true),
            ..Default::default()
        };
        let result =
            self.menu_dao
                .create(model)
                .await
                .map_err(|err: sea_orm::prelude::DbErr| {
                    error!("添加菜单信息失败, err: {:#?}", err);
                    Error::DbAddError.into_err_with_msg("添加菜单信息失败")
                })?;

        Ok(result)
    }

    /// 更新数据
    pub async fn update(&self, req: UpdateMenuReq) -> Result<u64, ErrorMsg> {
        let model = menu::ActiveModel {
            id: Set(req.id),
            pid: Set(req.pid),
            title: Set(req.title),
            icon_class: Set(req.icon_class),
            menu_type: Set(req.menu_type as i8),
            open_method: Set(req.open_method as i8),
            path: Set(req.path),
            component_path: Set(req.component_path),
            redirect_to: Set(req.redirect_to),
            link: Set(req.link),
            link_target: Set(req.link_target.map(|v| v.into())),
            is_hidden: Set(req.is_hidden),
            is_always_show_root: Set(req.is_always_show_root),
            permission: Set(req.permission),
            sort: Set(req.sort),
            desc: Set(req.desc),
            status: Set(true),
            ..Default::default()
        };

        let result = self.menu_dao.update(model).await.map_err(|err| {
            error!("更新菜单失败, err: {:#?}", err);
            Error::DbUpdateError.into_err_with_msg("更新菜单失败")
        })?;

        Ok(result)
    }

    /// 更新数据状态
    pub async fn update_status(&self, req: UpdateMenuStatusReq) -> Result<(), ErrorMsg> {
        self.menu_dao
            .update_status(req.id, req.status)
            .await
            .map_err(|err| {
                error!("更新菜单状态失败, err: {:#?}", err);
                Error::DbUpdateError.into_err_with_msg("更新菜单状态失败")
            })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, req: DeleteMenuReq) -> Result<u64, ErrorMsg> {
        let children = self.menu_dao.children(req.id).await.map_err(|err| {
            error!("获取所有子列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("获取所有子列表失败")
        })?;
        if !children.is_empty() {
            error!("请先删除子列表, children count: {:#?}", children.len());
            return Err(Error::DbDataExistChildrenError.into_err_with_msg("请先删除子列表"));
        }

        let result = self.menu_dao.delete(req.id).await.map_err(|err| {
            error!("删除菜单信息失败, err: {:#?}", err);
            Error::DbDeleteError.into_err_with_msg("删除菜单信息失败")
        })?;

        Ok(result)
    }
}

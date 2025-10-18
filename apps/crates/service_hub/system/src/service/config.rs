//! 配置管理

use log::error;
use nject::injectable;
use sea_orm::{DbErr::RecordNotUpdated, Set};

use database::utils::GenericTree;
use entity::system::config;
use err_code::{Error, ErrorMsg};

use crate::{
    dao::config::ConfigDao,
    dto::config::{
        CreateConfigReq, DeleteConfigReq, GetConfigReq, GetConfigsReq, UpdateConfigReq,
        UpdateConfigStatusReq,
    },
};

/// 服务层
#[injectable]
pub struct ConfigService {
    config_dao: ConfigDao,
}

impl ConfigService {
    /// 获取列表数据
    pub async fn list(&self, req: GetConfigsReq) -> Result<(Vec<config::Model>, u64), ErrorMsg> {
        // 获取所有数据
        if let Some(true) = req.all {
            return self.config_dao.all().await.map_err(|err| {
                error!("查询配置列表失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询配置列表失败")
            });
        }

        let (results, total) = self.config_dao.list(req).await.map_err(|err| {
            error!("查询配置列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询配置列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取树列表数据
    pub async fn tree(&self) -> Result<Vec<GenericTree<config::Model>>, ErrorMsg> {
        let (results, _total) = self.config_dao.all().await.map_err(|err| {
            error!("查询配置列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询配置列表失败")
        })?;

        // 将列表转换为树列表
        let results = GenericTree::to_tree(&results, None);

        Ok(results)
    }

    /// 获取详情数据
    pub async fn info(&self, req: GetConfigReq) -> Result<config::Model, ErrorMsg> {
        let result = self
            .config_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询配置信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询配置信息失败")
            })?
            .ok_or_else(|| {
                error!("配置不存在");
                Error::DbQueryEmptyError.into_err_with_msg("配置不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn create(&self, req: CreateConfigReq) -> Result<config::Model, ErrorMsg> {
        // 查询配置编码是否存在
        self.check_code_exist(req.code.clone(), None).await?;

        let model = config::ActiveModel {
            pid: Set(req.pid),
            name: Set(req.name),
            code: Set(req.code),
            value: Set(req.value),
            sort: Set(req.sort),
            desc: Set(req.desc),
            status: Set(true),
            ..Default::default()
        };
        let result = self.config_dao.create(model).await.map_err(|err| {
            error!("添加配置信息失败, err: {:#?}", err);
            Error::DbAddError.into_err_with_msg("添加配置信息失败")
        })?;

        Ok(result)
    }

    /// 更新配置
    pub async fn update(&self, req: UpdateConfigReq) -> Result<u64, ErrorMsg> {
        // 查询配置编码是否存在且不属于当前ID
        self.check_code_exist(req.code.clone(), Some(req.id))
            .await?;

        let model = config::ActiveModel {
            id: Set(req.id),
            pid: Set(req.pid),
            name: Set(req.name),
            code: Set(req.code),
            value: Set(req.value),
            sort: Set(req.sort),
            desc: Set(req.desc),
            status: Set(req.status),
            ..Default::default()
        };

        let result = self.config_dao.update(model).await.map_err(|err| {
            error!("更新配置失败, err: {:#?}", err);
            Error::DbUpdateError.into_err_with_msg("更新配置失败")
        })?;

        Ok(result)
    }

    /// 检查配置编码是否存在
    async fn check_code_exist(
        &self,
        code: String,
        current_id: Option<i32>,
    ) -> Result<(), ErrorMsg> {
        let result = self.config_dao.info_by_code(code).await.map_err(|err| {
            error!("查询配置编码失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询配置编码失败")
        })?;

        // 存在
        if let Some(model) = result
            && (current_id.is_none() || Some(model.id) != current_id)
        {
            error!("配置编码已存在");
            return Err(Error::DbDataExistError.into_err_with_msg("配置编码已存在"));
        }

        // 不存在
        Ok(())
    }

    /// 更新数据状态
    pub async fn update_status(&self, req: UpdateConfigStatusReq) -> Result<(), ErrorMsg> {
        self.config_dao
            .update_status(req.id, req.status)
            .await
            .map_err(|err| {
                if err == RecordNotUpdated {
                    error!("更新配置状态失败, 该配置不存在");
                    return Error::DbUpdateError
                        .into_err_with_msg("更新配置状态失败, 该配置不存在");
                }
                error!("更新配置状态失败, err: {:#?}", err);
                Error::DbUpdateError.into_err_with_msg("更新配置状态失败")
            })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, req: DeleteConfigReq) -> Result<u64, ErrorMsg> {
        let config_children = self.config_dao.children(req.id).await.map_err(|err| {
            error!("获取所有子列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("获取所有子列表失败")
        })?;
        if !config_children.is_empty() {
            error!(
                "请先删除子列表, children count: {:#?}",
                config_children.len()
            );
            return Err(Error::DbDataExistChildrenError.into_err_with_msg("请先删除子列表"));
        }

        let result = self.config_dao.delete(req.id).await.map_err(|err| {
            error!("删除配置信息失败, err: {:#?}", err);
            Error::DbDeleteError.into_err_with_msg("删除配置信息失败")
        })?;

        Ok(result)
    }
}

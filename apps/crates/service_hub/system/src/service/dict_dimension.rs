//! 字典维度管理

use log::error;
use nject::injectable;
use sea_orm::{DbErr::RecordNotUpdated, Set};

use entity::system::dict_dimension;
use err_code::{Error, ErrorMsg};

use crate::{
    dao::dict_dimension::DictDimensionDao,
    dto::dict_dimension::{
        CreateDictDimensionReq, DeleteDictDimensionReq, GetDictDimensionReq, GetDictDimensionsReq,
        UpdateDictDimensionReq, UpdateDictDimensionStatusReq,
    },
};

/// 服务层
#[injectable]
pub struct DictDimensionService {
    dict_dimension_dao: DictDimensionDao,
}

impl DictDimensionService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetDictDimensionsReq,
    ) -> Result<(Vec<dict_dimension::Model>, u64), ErrorMsg> {
        // 获取所有数据
        if let Some(true) = req.all {
            return self.dict_dimension_dao.all().await.map_err(|err| {
                error!("查询字典维度列表失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询字典维度列表失败")
            });
        }

        let (results, total) = self.dict_dimension_dao.list(req).await.map_err(|err| {
            error!("查询字典维度列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询字典维度列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, req: GetDictDimensionReq) -> Result<dict_dimension::Model, ErrorMsg> {
        let result = self
            .dict_dimension_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询字典维度信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询字典维度信息失败")
            })?
            .ok_or_else(|| {
                error!("字典维度不存在");
                Error::DbQueryEmptyError.into_err_with_msg("字典维度不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn create(
        &self,
        req: CreateDictDimensionReq,
    ) -> Result<dict_dimension::Model, ErrorMsg> {
        // 查询字典维度名称是否已存在
        self.check_name_exist(req.name.clone(), None).await?;

        // 查询字典维度编码是否存在
        self.check_code_exist(req.code.clone(), None).await?;

        let model = dict_dimension::ActiveModel {
            name: Set(req.name),
            code: Set(req.code),
            sort: Set(req.sort),
            desc: Set(req.desc),
            status: Set(true),
            ..Default::default()
        };
        let result = self.dict_dimension_dao.create(model).await.map_err(|err| {
            error!("添加字典维度信息失败, err: {:#?}", err);
            Error::DbAddError.into_err_with_msg("添加字典维度信息失败")
        })?;

        Ok(result)
    }

    /// 更新字典维度
    pub async fn update(&self, req: UpdateDictDimensionReq) -> Result<u64, ErrorMsg> {
        // 查询字典维度名称是否已存在且不属于当前ID
        self.check_name_exist(req.name.clone(), Some(req.id))
            .await?;

        // 查询字典维度编码是否存在且不属于当前ID
        self.check_code_exist(req.code.clone(), Some(req.id))
            .await?;

        let model = dict_dimension::ActiveModel {
            id: Set(req.id),
            name: Set(req.name),
            code: Set(req.code),
            sort: Set(req.sort),
            desc: Set(req.desc),
            status: Set(req.status),
            ..Default::default()
        };

        let result = self.dict_dimension_dao.update(model).await.map_err(|err| {
            error!("更新字典维度失败, err: {:#?}", err);
            Error::DbUpdateError.into_err_with_msg("更新字典维度失败")
        })?;

        Ok(result)
    }

    /// 查询字典维度名称是否已存在
    async fn check_name_exist(
        &self,
        name: String,
        current_id: Option<i32>,
    ) -> Result<(), ErrorMsg> {
        let result = self
            .dict_dimension_dao
            .info_by_code(name)
            .await
            .map_err(|err| {
                error!("查询字典维度名称失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询字典维度名称失败")
            })?;

        // 存在
        if let Some(model) = result
            && (current_id.is_none() || Some(model.id) != current_id)
        {
            error!("字典维度名称已存在");
            return Err(Error::DbDataExistError.into_err_with_msg("字典维度名称已存在"));
        }

        // 不存在
        Ok(())
    }

    /// 查询字典维度编码是否存在
    async fn check_code_exist(
        &self,
        code: String,
        current_id: Option<i32>,
    ) -> Result<(), ErrorMsg> {
        let result = self
            .dict_dimension_dao
            .info_by_code(code)
            .await
            .map_err(|err| {
                error!("查询字典维度编码失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询字典维度编码失败")
            })?;

        // 存在
        if let Some(model) = result
            && (current_id.is_none() || Some(model.id) != current_id)
        {
            error!("字典维度编码已存在");
            return Err(Error::DbDataExistError.into_err_with_msg("字典维度编码已存在"));
        }

        // 不存在
        Ok(())
    }

    /// 更新数据状态
    pub async fn update_status(&self, req: UpdateDictDimensionStatusReq) -> Result<(), ErrorMsg> {
        self.dict_dimension_dao
            .update_status(req.id, req.status)
            .await
            .map_err(|err| {
                if err == RecordNotUpdated {
                    error!("更新字典维度状态失败, 该字典维度不存在");
                    return Error::DbUpdateError
                        .into_err_with_msg("更新字典维度状态失败, 该字典维度不存在");
                }
                error!("更新字典维度状态失败, err: {:#?}", err);
                Error::DbUpdateError.into_err_with_msg("更新字典维度状态失败")
            })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, req: DeleteDictDimensionReq) -> Result<u64, ErrorMsg> {
        let result = self
            .dict_dimension_dao
            .delete(req.id)
            .await
            .map_err(|err| {
                error!("删除字典维度信息失败, err: {:#?}", err);
                Error::DbDeleteError.into_err_with_msg("删除字典维度信息失败")
            })?;

        Ok(result)
    }
}

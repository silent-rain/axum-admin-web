//! 模板管理

use log::error;
use nject::injectable;
use sea_orm::Set;

use err_code::{Error, ErrorMsg};

use crate::{
    dao::template::TemplateDao,
    dto::template::{
        BatchCreateTemplateReq, CreateTemplateReq, DeleteTemplateReq, GetTemplateReq,
        GetTemplatesReq, UpdateTemplateReq, UpdateTemplateStatusReq,
    },
    entity::template,
};

/// 服务层
#[injectable]
pub struct TemplateService {
    pub template_dao: TemplateDao,
}

impl TemplateService {
    /// 获取{{InterfaceName}}列表
    pub async fn list(
        &self,
        req: GetTemplatesReq,
    ) -> Result<(Vec<template::Model>, u64), ErrorMsg> {
        if req.is_all {
            let (results, total) = self.template_dao.all().await.map_err(|err| {
                error!("查询{{InterfaceName}}列表失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询{{InterfaceName}}列表失败")
            })?;

            return Ok((results, total));
        }

        let (results, total) = self.template_dao.list(req).await.map_err(|err| {
            error!("查询{{InterfaceName}}列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询{{InterfaceName}}列表失败")
        })?;
        Ok((results, total))
    }

    /// 获取{{InterfaceName}}详情
    pub async fn info(&self, req: GetTemplateReq) -> Result<template::Model, ErrorMsg> {
        let result = self
            .template_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询{{InterfaceName}}信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询{{InterfaceName}}信息失败")
            })?
            .ok_or_else(|| {
                error!("{{InterfaceName}}不存在");
                Error::DbQueryEmptyError.into_err_with_msg("{{InterfaceName}}不存在")
            })?;

        Ok(result)
    }

    /// 添加{{InterfaceName}}
    pub async fn create(&self, data: CreateTemplateReq) -> Result<template::Model, ErrorMsg> {
        let model = template::ActiveModel {
            user_id: Set(data.user_id),
            desc: Set(data.desc),
            ..Default::default()
        };

        let result = self.template_dao.create(model).await.map_err(|err| {
            error!("添加{{InterfaceName}}失败, err: {:#?}", err);
            Error::DbAddError.into_err_with_msg("添加{{InterfaceName}}失败")
        })?;

        Ok(result)
    }

    /// 批量添加{{InterfaceName}}
    pub async fn batch_create(&self, data: BatchCreateTemplateReq) -> Result<i32, ErrorMsg> {
        let mut models = Vec::new();
        for item in data.data {
            let model = template::ActiveModel {
                user_id: Set(item.user_id),
                desc: Set(item.desc),
                status: Set(item.status),
                ..Default::default()
            };
            models.push(model);
        }

        let result = self
            .template_dao
            .batch_create(models)
            .await
            .map_err(|err| {
                error!("批量添加{{InterfaceName}}失败, err: {:#?}", err);
                Error::DbBatchAddError.into_err_with_msg("批量添加{{InterfaceName}}失败")
            })?;

        Ok(result)
    }

    /// 更新{{InterfaceName}}
    pub async fn update(&self, data: UpdateTemplateReq) -> Result<u64, ErrorMsg> {
        let model = template::ActiveModel {
            id: Set(data.id),
            desc: Set(data.desc),
            status: Set(data.status),
            ..Default::default()
        };

        let result = self.template_dao.update(model).await.map_err(|err| {
            error!("更新{{InterfaceName}}失败, err: {:#?}", err);
            Error::DbUpdateError.into_err_with_msg("更新{{InterfaceName}}失败")
        })?;

        Ok(result)
    }

    /// 更新{{InterfaceName}}状态
    pub async fn update_status(
        &self,
        req: UpdateTemplateStatusReq,
    ) -> Result<template::Model, ErrorMsg> {
        let result = self
            .template_dao
            .status(req.id, req.status)
            .await
            .map_err(|err| {
                error!("更新{{InterfaceName}}状态失败, err: {:#?}", err);
                Error::DbUpdateError.into_err_with_msg("更新{{InterfaceName}}状态失败")
            })?;

        Ok(result)
    }

    /// 删除{{InterfaceName}}
    pub async fn delete(&self, req: DeleteTemplateReq) -> Result<u64, ErrorMsg> {
        let result = self.template_dao.delete(req.id).await.map_err(|err| {
            error!("删除{{InterfaceName}}失败, err: {:#?}", err);
            Error::DbDeleteError.into_err_with_msg("删除{{InterfaceName}}失败")
        })?;

        Ok(result)
    }

    /// 批量删除{{InterfaceName}}
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, ErrorMsg> {
        let result = self.template_dao.batch_delete(ids).await.map_err(|err| {
            error!("批量删除{{InterfaceName}}失败, err: {:#?}", err);
            Error::DbBatchDeleteError.into_err_with_msg("批量删除{{InterfaceName}}失败")
        })?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use database::mock::Mock;
    use orm_migration::template::app_template;
    use orm_migration::user::user_base;

    #[tokio::test]
    async fn test_mock_add() -> Result<(), ErrorMsg> {
        let pool = Mock::builder()
            .await
            .map_err(|err| Error::DbInit(err.to_string()).into_err())?
            .migration_migrations(vec![&user_base::Migration, &app_template::Migration])
            .await
            .map_err(|err| Error::DbTableMigration(err.to_string()).into_err())?
            .build();

        let dao = TemplateDao { db: pool };

        let service = TemplateService { template_dao: dao };

        // 添加模板1
        let data = CreateTemplateReq {
            user_id: 1,
            desc: Some("desc".to_string()),
        };
        let result = service.create(data).await?;
        println!("create result1: {result:#?}");
        assert!(result.user_id == 1);

        // 添加模板2
        let data = CreateTemplateReq {
            user_id: 2,
            desc: Some("desc".to_string()),
        };
        let result = service.create(data).await?;
        println!("create result2: {result:#?}");
        assert!(result.user_id == 2);

        // 查询模板1
        let result = service.info(GetTemplateReq { id: 1 }).await?;
        println!("info result: {result:#?}");
        assert!(result.user_id == 1);

        // 查询所有的模板
        let query = GetTemplatesReq {
            is_all: true,
            ..Default::default()
        };
        let (results, total) = service.list(query).await?;
        println!("all results: {results:#?}");
        assert!(!results.is_empty());
        assert!(total == 2);

        Ok(())
    }
}

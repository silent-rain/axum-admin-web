//! 模板管理

use std::{str::FromStr, sync::Arc};

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

use database::{Pagination, PoolTrait};

use crate::{
    dto::template::GetAppTemplatesReq,
    entity::{TemplateEntity, template},
};

/// 数据访问
#[injectable]
pub struct AppTemplateDao {
    pub db: Arc<dyn PoolTrait>,
}

impl AppTemplateDao {
    /// 获取所有数据
    pub async fn all(&self) -> Result<(Vec<template::Model>, u64), DbErr> {
        let results = TemplateEntity::find()
            .order_by_asc(template::Column::Id)
            .all(self.db.db())
            .await?;
        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetAppTemplatesReq,
    ) -> Result<(Vec<template::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = TemplateEntity::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(template::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(template::Column::CreatedAt.lt(v))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let order_by_col = match req.order_by {
            Some(v) => template::Column::from_str(&v).map_or(template::Column::Id, |v| v),
            None => template::Column::Id,
        };

        let results = states
            .order_by_desc(order_by_col)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<Option<template::Model>, DbErr> {
        TemplateEntity::find_by_id(id).one(self.db.db()).await
    }

    /// 添加数据
    pub async fn create(
        &self,
        active_model: template::ActiveModel,
    ) -> Result<template::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 批量添加数据
    pub async fn batch_create(
        &self,
        active_models: Vec<template::ActiveModel>,
    ) -> Result<i32, DbErr> {
        let result = TemplateEntity::insert_many(active_models)
            .exec(self.db.db())
            .await?;
        Ok(result.last_insert_id)
    }

    /// 更新数据
    pub async fn update(&self, active_model: template::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = TemplateEntity::update_many()
            .set(active_model)
            .filter(template::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn status(&self, id: i32, status: bool) -> Result<template::Model, DbErr> {
        let active_model = template::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let result = active_model.update(self.db.db()).await?;
        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = TemplateEntity::delete_by_id(id).exec(self.db.db()).await?;
        Ok(result.rows_affected)
    }

    /// 批量删除数据
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, DbErr> {
        let result = TemplateEntity::delete_many()
            .filter(template::Column::Id.is_in(ids))
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use database::mock::Mock;
    use orm_migration::template::app_template::Migration as MAppTemplate;
    use orm_migration::user::user_base::Migration as MUserBase;

    use sea_orm::DbBackend;

    #[test]
    fn test_all() {
        let result = TemplateEntity::find()
            .order_by_asc(template::Column::Id)
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"SELECT `t_template`.`id`, `t_template`.`user_id`, `t_template`.`desc`, `t_template`.`status`, `t_template`.`created_at`, `t_template`.`updated_at` FROM `t_template` ORDER BY `t_template`.`id` ASC"#;

        assert_eq!(result, sql);
    }

    #[test]
    fn test_list() {}

    #[test]
    fn test_info() {
        let result = TemplateEntity::find_by_id(1)
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"SELECT `t_template`.`id`, `t_template`.`user_id`, `t_template`.`desc`, `t_template`.`status`, `t_template`.`created_at`, `t_template`.`updated_at` FROM `t_template` WHERE `t_template`.`id` = 1"#;

        assert_eq!(result, sql);
    }

    #[test]
    fn test_add() {
        let active_model = template::ActiveModel {
            id: Set(1),
            user_id: Set(11),
            status: Set(true),
            ..Default::default()
        };
        let result = TemplateEntity::insert(active_model)
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"INSERT INTO `t_template` (`id`, `user_id`, `status`) VALUES (1, 11, TRUE)"#;

        assert_eq!(result, sql);
    }

    #[test]
    fn test_batch_add() {
        let active_model1 = template::ActiveModel {
            id: Set(1),
            user_id: Set(11),
            status: Set(true),
            ..Default::default()
        };
        let active_model2 = template::ActiveModel {
            id: Set(2),
            user_id: Set(22),
            status: Set(false),
            ..Default::default()
        };
        let models = [active_model1, active_model2];
        let result = TemplateEntity::insert_many(models)
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"INSERT INTO `t_template` (`id`, `user_id`, `status`) VALUES (1, 11, TRUE), (2, 22, FALSE)"#;

        assert_eq!(result, sql);
    }

    #[test]
    fn test_update() {
        let active_model = template::ActiveModel {
            id: Set(1),
            user_id: Set(11),
            status: Set(true),
            ..Default::default()
        };
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = TemplateEntity::update_many()
            .set(active_model)
            .filter(template::Column::Id.eq(id))
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"UPDATE `t_template` SET `id` = 1, `user_id` = 11, `status` = TRUE WHERE `t_template`.`id` = 1"#;

        assert_eq!(result, sql);
    }

    #[test]
    fn test_status() {
        let active_model = template::ActiveModel {
            id: Set(1),
            status: Set(false),
            ..Default::default()
        };
        let result = TemplateEntity::update(active_model)
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"UPDATE `t_template` SET `status` = FALSE WHERE `t_template`.`id` = 1"#;

        assert_eq!(result, sql);
    }

    #[test]
    fn test_delete() {
        let result = TemplateEntity::delete_by_id(1)
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"DELETE FROM `t_template` WHERE `t_template`.`id` = 1"#;

        assert_eq!(result, sql);
    }

    #[test]
    fn test_batch_delete() {
        let ids = vec![1, 2, 3, 4];
        let result = TemplateEntity::delete_many()
            .filter(template::Column::Id.is_in(ids))
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"DELETE FROM `t_template` WHERE `t_template`.`id` IN (1, 2, 3, 4)"#;

        assert_eq!(result, sql);
    }

    #[tokio::test]
    async fn test_mock_all() -> Result<(), DbErr> {
        let pool = Mock::builder()
            .await?
            .migration_migrations(vec![&MUserBase, &MAppTemplate])
            .await?
            .build();

        let dao = AppTemplateDao { db: pool };

        let (results, total) = dao.all().await?;
        assert!(results.is_empty());
        assert!(total == 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_mock_info() -> Result<(), Box<DbErr>> {
        let pool = Mock::builder()
            .await?
            .migration_migrations(vec![&MUserBase, &MAppTemplate])
            .await?
            .build();

        let dao = AppTemplateDao { db: pool };

        let result = dao.info(1).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_mock_add() -> Result<(), DbErr> {
        let pool = Mock::builder()
            .await?
            .migration_migrations(vec![&MUserBase, &MAppTemplate])
            .await?
            .build();

        let dao = AppTemplateDao { db: pool };

        // 添加模板1
        let active_model = template::ActiveModel {
            user_id: Set(1),
            desc: Set(Some("desc".to_string())),
            status: Set(true),
            ..Default::default()
        };
        let result = dao.create(active_model).await?;
        println!("create result1: {result:#?}");
        assert!(result.user_id == 1);

        // 添加模板2
        let active_model = template::ActiveModel {
            user_id: Set(2),
            desc: Set(Some("desc2".to_string())),
            status: Set(false),
            ..Default::default()
        };
        let result = dao.create(active_model).await?;
        println!("create result2: {result:#?}");
        assert!(result.user_id == 2);

        // 查询模板1
        let result = dao.info(1).await?;
        println!("info result: {result:#?}");
        assert!(result.is_some());

        // 查询所有的模板
        let (results, total) = dao.all().await?;
        println!("all results: {results:#?}");
        assert!(!results.is_empty());
        assert!(total == 2);

        Ok(())
    }
}

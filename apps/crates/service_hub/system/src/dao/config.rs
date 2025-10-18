//! 配置管理
use std::sync::Arc;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

use database::{Pagination, PoolTrait};
use entity::system::{ConfigEntity, config};

use crate::dto::config::GetConfigsReq;

/// 数据访问
#[injectable]
pub struct ConfigDao {
    db: Arc<dyn PoolTrait>,
}

impl ConfigDao {
    /// 获取所有数据
    pub async fn all(&self) -> Result<(Vec<config::Model>, u64), DbErr> {
        let results = ConfigEntity::find()
            .order_by_asc(config::Column::Id)
            .all(self.db.db())
            .await?;
        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取数据列表
    pub async fn list(&self, req: GetConfigsReq) -> Result<(Vec<config::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = ConfigEntity::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(config::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(config::Column::CreatedAt.lt(v))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(config::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取父ID下的所有子列表
    pub async fn children(&self, pid: i32) -> Result<Vec<config::Model>, DbErr> {
        ConfigEntity::find()
            .filter(config::Column::Pid.eq(pid))
            .all(self.db.db())
            .await
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<config::Model>, DbErr> {
        ConfigEntity::find_by_id(id).one(self.db.db()).await
    }

    /// 通过配置编码获取详情信息
    pub async fn info_by_code(&self, code: String) -> Result<Option<config::Model>, DbErr> {
        ConfigEntity::find()
            .filter(config::Column::Code.eq(code))
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn create(&self, active_model: config::ActiveModel) -> Result<config::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新数据
    pub async fn update(&self, active_model: config::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = ConfigEntity::update_many()
            .set(active_model)
            .filter(config::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn update_status(&self, id: i32, status: bool) -> Result<(), DbErr> {
        let active_model = config::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.db()).await?;
        Ok(())
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = ConfigEntity::delete_by_id(id).exec(self.db.db()).await?;
        Ok(result.rows_affected)
    }
}

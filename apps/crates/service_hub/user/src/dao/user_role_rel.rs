//! 用户角色关系管理

use std::sync::Arc;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

use database::{Pagination, PoolTrait};
use entity::user::{UserRoleRelEntity, user_role_rel};

use crate::dto::user_role_rel::GetUserRoleRelsReq;

/// 数据访问
#[injectable]
pub struct UserRoleRelDao {
    db: Arc<dyn PoolTrait>,
}

impl UserRoleRelDao {
    /// 获取所有数据
    pub async fn all(&self) -> Result<(Vec<user_role_rel::Model>, u64), DbErr> {
        let results = UserRoleRelEntity::find()
            .order_by_asc(user_role_rel::Column::Id)
            .all(self.db.db())
            .await?;
        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetUserRoleRelsReq,
    ) -> Result<(Vec<user_role_rel::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = UserRoleRelEntity::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(user_role_rel::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(user_role_rel::Column::CreatedAt.lt(v))
            })
            .apply_if(req.user_id, |query, v| {
                query.filter(user_role_rel::Column::UserId.eq(v))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(user_role_rel::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 添加数据
    pub async fn create(
        &self,
        active_model: user_role_rel::ActiveModel,
    ) -> Result<user_role_rel::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 批量添加数据
    pub async fn batch_create(
        &self,
        active_models: Vec<user_role_rel::ActiveModel>,
    ) -> Result<i32, DbErr> {
        let result = UserRoleRelEntity::insert_many(active_models)
            .exec(self.db.db())
            .await?;
        Ok(result.last_insert_id)
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = UserRoleRelEntity::delete_many()
            .filter(user_role_rel::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }

    /// 批量删除数据
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, DbErr> {
        let result = UserRoleRelEntity::delete_many()
            .filter(user_role_rel::Column::Id.is_in(ids))
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }
}

impl UserRoleRelDao {
    /// 获取数据列表
    pub async fn list_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<(Vec<user_role_rel::Model>, u64), DbErr> {
        let results = UserRoleRelEntity::find()
            .filter(user_role_rel::Column::UserId.eq(user_id))
            .all(self.db.db())
            .await?;

        let total = results.len() as u64;
        Ok((results, total))
    }
}

//! 用户session管理
use std::sync::Arc;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

use database::{Pagination, PoolTrait};
use entity::user::{UserSessionEntity, user_session};

use crate::dto::user_session::GetUserSessionsReq;

/// 数据访问
#[injectable]
pub struct UserSessionDao {
    db: Arc<dyn PoolTrait>,
}

impl UserSessionDao {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetUserSessionsReq,
    ) -> Result<(Vec<user_session::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = UserSessionEntity::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(user_session::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(user_session::Column::CreatedAt.lt(v))
            })
            .apply_if(req.session_id, |query, v| {
                query.filter(user_session::Column::SessionId.like(format!("{v}%")))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(user_session::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<user_session::Model>, DbErr> {
        UserSessionEntity::find_by_id(id).one(self.db.db()).await
    }

    /// 添加详情信息
    pub async fn create(
        &self,
        active_model: user_session::ActiveModel,
    ) -> Result<user_session::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新信息
    pub async fn update(&self, active_model: user_session::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = UserSessionEntity::update_many()
            .set(active_model)
            .filter(user_session::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn update_status(&self, id: i32, status: bool) -> Result<(), DbErr> {
        let active_model = user_session::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.db()).await?;
        Ok(())
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = UserSessionEntity::delete_by_id(id)
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }
}

impl UserSessionDao {
    /// 通过nsession_id获取详情信息
    pub async fn info_by_session_id(
        &self,
        session_id: String,
    ) -> Result<Option<user_session::Model>, DbErr> {
        UserSessionEntity::find()
            .filter(user_session::Column::SessionId.eq(session_id))
            .one(self.db.db())
            .await
    }
}

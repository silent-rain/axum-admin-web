//! 数据操作层

use std::sync::Arc;

use chrono::Local;
use database::PoolTrait;
use entity::user::{UserSessionEntity, user_session};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, Set};

pub struct UserSessionDao {
    db: Arc<dyn PoolTrait>,
}

impl UserSessionDao {
    pub fn new(db: Arc<dyn PoolTrait>) -> Self {
        UserSessionDao { db }
    }

    /// 通过nsession_id获取详情信息
    pub async fn info(&self, session_id: String) -> Result<Option<user_session::Model>, DbErr> {
        UserSessionEntity::find()
            .filter(user_session::Column::SessionId.eq(session_id))
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn create(
        &self,
        active_model: user_session::ActiveModel,
    ) -> Result<user_session::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新信息
    pub async fn update(
        &self,
        session_id: String,
        active_model: user_session::ActiveModel,
    ) -> Result<u64, DbErr> {
        let result = UserSessionEntity::update_many()
            .set(active_model)
            .filter(user_session::Column::SessionId.eq(session_id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 删除session
    pub async fn delete(&self, session_id: String) -> Result<u64, DbErr> {
        let active_model = user_session::ActiveModel {
            status: Set(false),
            ..Default::default()
        };

        let result = UserSessionEntity::update_many()
            .set(active_model)
            .filter(user_session::Column::SessionId.eq(session_id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    pub async fn delete_expired(&self) -> Result<u64, DbErr> {
        let active_model = user_session::ActiveModel {
            status: Set(false),
            ..Default::default()
        };

        let result = UserSessionEntity::update_many()
            .set(active_model)
            .filter(user_session::Column::ExpiryDate.lt(Local::now().naive_local()))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }
}

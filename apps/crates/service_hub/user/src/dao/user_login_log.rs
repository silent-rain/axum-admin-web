//! 登陆日志管理

use std::sync::Arc;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

use database::{Pagination, PoolTrait};
use entity::user::{UserLoginLogEntity, user_login_log};

use crate::dto::user_login_log::GetUserLoginLogsReq;

/// 数据访问
#[injectable]
pub struct UserLoginLogDao {
    db: Arc<dyn PoolTrait>,
}

impl UserLoginLogDao {
    pub fn new(db: Arc<dyn PoolTrait>) -> Self {
        UserLoginLogDao { db }
    }

    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetUserLoginLogsReq,
    ) -> Result<(Vec<user_login_log::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = UserLoginLogEntity::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(user_login_log::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(user_login_log::Column::CreatedAt.lt(v))
            })
            .apply_if(req.user_id, |query, v| {
                query.filter(user_login_log::Column::UserId.eq(v))
            })
            .apply_if(req.username, |query, v| {
                query.filter(user_login_log::Column::Username.like(format!("{v}%")))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(user_login_log::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<user_login_log::Model>, DbErr> {
        UserLoginLogEntity::find_by_id(id).one(self.db.db()).await
    }

    /// 根据SessionId获取详情信息
    pub async fn info_by_session_id(
        &self,
        session_id: String,
    ) -> Result<Option<user_login_log::Model>, DbErr> {
        UserLoginLogEntity::find()
            .filter(user_login_log::Column::SessionId.eq(session_id))
            .order_by_desc(user_login_log::Column::Id)
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn create(
        &self,
        active_model: user_login_log::ActiveModel,
    ) -> Result<user_login_log::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新数据
    pub async fn update(&self, active_model: user_login_log::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = UserLoginLogEntity::update_many()
            .set(active_model)
            .filter(user_login_log::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新禁用状态
    pub async fn update_status(&self, id: i32, login_status: i8) -> Result<(), DbErr> {
        let active_model = user_login_log::ActiveModel {
            id: Set(id),
            login_status: Set(login_status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.db()).await?;
        Ok(())
    }
}

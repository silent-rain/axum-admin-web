//! 任务调度状态日志管理

use std::sync::Arc;

use crate::dto::schedule_status_log::GetScheduleStatusLogListLogReq;

use database::{Pagination, PoolTrait};
use entity::schedule::{schedule_status_log, ScheduleStatusLog};

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

/// 数据访问
#[injectable]
pub struct ScheduleStatusLogDao {
    db: Arc<dyn PoolTrait>,
}

impl ScheduleStatusLogDao {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetScheduleStatusLogListLogReq,
    ) -> Result<(Vec<schedule_status_log::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = ScheduleStatusLog::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(schedule_status_log::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(schedule_status_log::Column::CreatedAt.lt(v))
            })
            .apply_if(req.job_id, |query, v| {
                query.filter(schedule_status_log::Column::JobId.eq(v))
            })
            .apply_if(req.job_id, |query, v| {
                query.filter(schedule_status_log::Column::JobId.eq(v))
            })
            .apply_if(req.status, |query, v| {
                query.filter(schedule_status_log::Column::Status.eq(v))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(schedule_status_log::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<schedule_status_log::Model>, DbErr> {
        ScheduleStatusLog::find_by_id(id).one(self.db.db()).await
    }

    /// 获取最新的UUID数据
    pub async fn last_by_job_id(
        &self,
        job_id: i32,
    ) -> Result<Option<schedule_status_log::Model>, DbErr> {
        ScheduleStatusLog::find()
            .filter(schedule_status_log::Column::JobId.eq(job_id))
            .order_by_desc(schedule_status_log::Column::Id)
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: schedule_status_log::ActiveModel,
    ) -> Result<schedule_status_log::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新数据
    pub async fn update(
        &self,
        active_model: schedule_status_log::ActiveModel,
    ) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = ScheduleStatusLog::update_many()
            .set(active_model)
            .filter(schedule_status_log::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), DbErr> {
        let active_model = schedule_status_log::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.db()).await?;
        Ok(())
    }

    /// 按主键删除
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = ScheduleStatusLog::delete_by_id(id)
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }
}

//! 任务调度事件日志表

use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EnumIter,
    PrimaryKeyTrait, prelude::DateTime,
};
use serde::{Deserialize, Serialize};

/// 任务调度事件日志表
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_schedule_event_log")]
pub struct Model {
    /// 事件日志ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 任务ID
    pub job_id: i32,
    /// 任务调度ID
    pub uuid: String,
    /// 任务状态(0:开始,1:完成,2:停止,3:移除)
    pub status: i16,
    /// 创建时间
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

//! 职级表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

/// 职级表
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_org_rank")]
pub struct Model {
    /// 职级ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 职级名称
    pub name: String,
    /// 职级等级
    pub level: u16,
    /// 排序
    pub sort: Option<i32>,
    /// 职级描述
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
    /// 创建时间
    pub created_at: DateTimeLocal,
    /// 更新时间
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

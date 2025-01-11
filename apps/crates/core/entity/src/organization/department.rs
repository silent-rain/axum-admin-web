//! 部门表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

use crate::utils::list_tree::GenericTreeTrait;

/// 部门表
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_org_department")]
pub struct Model {
    /// 部门ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 上级部门ID
    pub pid: Option<i32>,
    /// 所有上级部门ID, 用逗号分开
    pub pids: Option<String>,
    /// 部门名称
    pub name: String,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
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

/// 实现 `GenericTreeTrait` trait, 将列表数据转换为树结构
impl GenericTreeTrait for Model {
    fn id(&self) -> i32 {
        self.id
    }

    fn pid(&self) -> Option<i32> {
        self.pid
    }
}

//! OpenApi接口表

use chrono::Local;
use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EnumIter, PrimaryKeyTrait, Set,
    prelude::{DateTime, async_trait::async_trait},
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use database::utils::GenericTreeTrait;

/// OpenApi接口表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_perm_openapi")]
pub struct Model {
    /// 自增ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 父ID
    pub pid: Option<i32>,
    /// 类别,0:目录,1:接口
    pub category: i8,
    /// 接口名称
    pub name: String,
    /// 请求类型
    pub method: String,
    /// 资源路径
    pub path: String,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
    /// 创建时间
    pub created_at: DateTime,
    /// 更新时间
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// Will be triggered before insert / update
    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.updated_at = Set(Local::now().naive_local());
        Ok(self)
    }
}

/// 实现 `GenericTreeTrait` trait, 将列表数据转换为树结构
impl GenericTreeTrait for Model {
    fn id(&self) -> i32 {
        self.id
    }

    fn pid(&self) -> Option<i32> {
        self.pid
    }
}

pub mod enums {
    use super::*;

    /// OpenApi接口类别
    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum Category {
        /// 目录
        Directory = 0,
        /// 接口
        Interface = 1,
    }
}

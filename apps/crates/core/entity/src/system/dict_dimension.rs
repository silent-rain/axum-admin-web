//! 字典维度表

use chrono::Local;
use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
    Set,
    prelude::{DateTime, async_trait::async_trait},
};
use serde::{Deserialize, Serialize};

/// 字典维度表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_sys_dict_dimension")]
pub struct Model {
    /// 字典维度ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 字典维度名称
    #[sea_orm(unique)]
    pub name: String,
    /// 字典维度编码
    #[sea_orm(unique)]
    pub code: String,
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
pub enum Relation {
    #[sea_orm(has_many = "super::dict_data::Entity")]
    SysDictData,
}

impl Related<super::dict_data::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysDictData.def()
    }
}

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

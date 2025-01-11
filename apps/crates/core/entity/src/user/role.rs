//! 角色表

use chrono::Local;
use sea_orm::{
    prelude::{async_trait::async_trait, DateTime},
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
    Set,
};
use serde::{Deserialize, Serialize};

/// 角色表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_user_role")]
pub struct Model {
    /// 角色ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 角色名称
    #[sea_orm(unique)]
    pub name: String,
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
    #[sea_orm(has_many = "crate::permission::menu_role_rel::Entity")]
    PermMenuRoleRel,
    #[sea_orm(has_many = "crate::user::user_role_rel::Entity")]
    UserRoleRel,
}

impl Related<crate::permission::menu_role_rel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermMenuRoleRel.def()
    }
}

impl Related<crate::user::user_role_rel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRoleRel.def()
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

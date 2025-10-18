//! 用户角色关系表

use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, EntityTrait, EnumIter,
    ForeignKeyAction, PrimaryKeyTrait, Related, RelationDef, RelationTrait, prelude::DateTime,
};
use serde::{Deserialize, Serialize};

/// 用户角色关系表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_user_role_rel")]
pub struct Model {
    /// 自增ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 用户ID
    pub user_id: i32,
    /// 角色ID
    pub role_id: i32,
    /// 创建时间
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Base,
    Role,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Base => Entity::belongs_to(super::user_base::Entity)
                .from(Column::UserId)
                .to(super::user_base::Column::Id)
                .on_update(ForeignKeyAction::Cascade)
                .on_delete(ForeignKeyAction::Cascade)
                .into(),
            Self::Role => {
                // 检查关系是否属于实体
                Entity::belongs_to(super::role::Entity)
                    // 从实体建立关系
                    .from(Column::RoleId)
                    // 与实体建立关系
                    .to(super::role::Column::Id)
                    // 发生更新操作时对外键执行的操作
                    .on_update(ForeignKeyAction::Cascade)
                    // 发生删除操作时对外键执行的操作
                    .on_delete(ForeignKeyAction::Cascade)
                    .into()
            }
        }
    }
}

impl Related<super::user_base::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Base.def()
    }
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

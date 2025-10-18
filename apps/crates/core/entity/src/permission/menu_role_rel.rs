//! 菜单角色关系表

use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, EntityTrait, EnumIter,
    ForeignKeyAction, PrimaryKeyTrait, Related, RelationDef, RelationTrait, prelude::DateTime,
};
use serde::{Deserialize, Serialize};

use crate::user::role;

/// 菜单角色关系表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_perm_menu_role_rel")]
pub struct Model {
    /// 自增ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 菜单ID
    pub menu_id: i32,
    /// 角色ID
    pub role_id: i32,
    /// 创建时间
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Menu,
    Role,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Menu => Entity::belongs_to(super::menu::Entity)
                .from(Column::MenuId)
                .to(super::menu::Column::Id)
                .on_update(ForeignKeyAction::Cascade)
                .on_delete(ForeignKeyAction::Cascade)
                .into(),
            Self::Role => Entity::belongs_to(role::Entity)
                .from(Column::RoleId)
                .to(role::Column::Id)
                .on_update(ForeignKeyAction::Cascade)
                .on_delete(ForeignKeyAction::Cascade)
                .into(),
        }
    }
}

impl Related<super::menu::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Menu.def()
    }
}

impl Related<role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

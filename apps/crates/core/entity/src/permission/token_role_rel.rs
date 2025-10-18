//! 令牌角色关系表

use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, EntityTrait, EnumIter,
    ForeignKeyAction, PrimaryKeyTrait, Related, RelationDef, RelationTrait, prelude::DateTime,
};
use serde::{Deserialize, Serialize};

use crate::user::role;

/// 令牌角色关系表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_perm_token_role_rel")]
pub struct Model {
    /// 自增ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 令牌ID
    pub token_id: i32,
    /// 角色ID
    pub role_id: i32,
    /// 创建时间
    pub created_at: DateTime,
}

// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {
//     #[sea_orm(
//         belongs_to = "super::token::Entity",
//         from = "Column::TokenId",
//         to = "super::token::Column::Id",
//         on_update = "Cascade",
//         on_delete = "Cascade"
//     )]
//     PermToken,
//     #[sea_orm(
//         belongs_to = "role::Entity",
//         from = "Column::RoleId",
//         to = "role::Column::Id",
//         on_update = "Cascade",
//         on_delete = "Cascade"
//     )]
//     UserRole,
// }

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Token,
    Role,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Token => Entity::belongs_to(super::token::Entity)
                .from(Column::TokenId)
                .to(super::token::Column::Id)
                .on_update(ForeignKeyAction::Cascade)
                .on_delete(ForeignKeyAction::Cascade)
                .into(),
            Self::Role => {
                Entity::belongs_to(role::Entity)
                    // 从实体建立关系
                    .from(Column::RoleId)
                    // 与实体建立关系
                    .to(role::Column::Id)
                    // 发生更新操作时对外键执行的操作
                    .on_update(ForeignKeyAction::Cascade)
                    // 发生删除操作时对外键执行的操作
                    .on_delete(ForeignKeyAction::Cascade)
                    .into()
            }
        }
    }
}

impl Related<super::token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Token.def()
    }
}

impl Related<role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

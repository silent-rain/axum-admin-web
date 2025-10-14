//! 角色表

use chrono::Local;
use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait, Set,
    prelude::{DateTime, async_trait::async_trait},
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

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    UserBase,
    PermMenuRoleRel,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::UserBase => Entity::has_many(super::user_base::Entity).into(),
            Self::PermMenuRoleRel => {
                Entity::has_many(crate::permission::menu_role_rel::Entity).into()
            }
        }
    }
}

impl Related<super::user_base::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_role_rel::Relation::UserBase.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::user_role_rel::Relation::UserRole.def().rev())
    }
}

impl Related<crate::permission::menu_role_rel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PermMenuRoleRel.def()
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

#[cfg(test)]
mod tests {
    use sea_orm::{
        ColumnTrait, DbBackend, JoinType, QueryFilter, QueryOrder, QuerySelect, QueryTrait,
        RelationTrait,
    };

    use crate::user::{Role, UserBase, UserRoleRel, role, user_base, user_role_rel};

    use super::*;

    #[test]
    fn test_roles() {
        let result = Role::find()
            .select_only()
            .columns([role::Column::Id])
            .join(JoinType::InnerJoin, role::Relation::UserBase.def())
            .filter(user_role_rel::Column::UserId.eq(10))
            .order_by_asc(role::Column::Id)
            .build(DbBackend::Postgres)
            .to_string();

        let sql = r#"SELECT "t_user_role"."id" FROM "t_user_role" INNER JOIN "t_user_role_rel" ON "t_user_role"."id" = "t_user_role_rel"."role_id" WHERE "t_user_role_rel"."user_id" = 10 ORDER BY "t_user_role"."id" ASC"#;
        assert_eq!(result, sql);
    }

    #[test]
    fn test_roles2() {
        let result = UserRoleRel::find()
            .select_only()
            .columns([role::Column::Id])
            .join(JoinType::InnerJoin, user_role_rel::Relation::UserRole.def())
            .filter(user_role_rel::Column::UserId.eq(10))
            .order_by_asc(role::Column::Id)
            .build(DbBackend::Postgres)
            .to_string();

        let sql = r#"SELECT "t_user_role"."id" FROM "t_user_role_rel" INNER JOIN "t_user_role" ON "t_user_role_rel"."role_id" = "t_user_role"."id" WHERE "t_user_role_rel"."user_id" = 10 ORDER BY "t_user_role"."id" ASC"#;
        assert_eq!(result, sql);
    }

    #[test]
    fn test_left_join() {
        let result = Role::find()
            .select_only()
            .columns([role::Column::Id])
            .left_join(UserBase)
            .filter(user_base::Column::Id.eq(10))
            .order_by_asc(role::Column::Id)
            .build(DbBackend::Postgres)
            .to_string();

        let sql = r#"SELECT "t_user_role"."id" FROM "t_user_role" LEFT JOIN "t_user_role_rel" ON "t_user_role"."id" = "t_user_role_rel"."role_id" LEFT JOIN "t_user_base" ON "t_user_role_rel"."user_id" = "t_user_base"."id" WHERE "t_user_base"."id" = 10 ORDER BY "t_user_role"."id" ASC"#;
        assert_eq!(result, sql);
    }
}

//! 用户角色关系表
//! Entity: [`entity::user::UserRoleRel`]

use sea_orm::{
    DatabaseBackend, DeriveIden, DeriveMigrationName, Iden,
    sea_query::{ColumnDef, Expr, ForeignKey, ForeignKeyAction, Table},
};
use sea_orm_migration::{DbErr, MigrationTrait, SchemaManager, async_trait};

use crate::{
    user::{role::Role, user_base::UserBase},
    utils::if_not_exists_create_unique_index,
};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(UserRoleRel::Table)
                    .comment("用户角色关系表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserRoleRel::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("ID"),
                    )
                    .col(
                        ColumnDef::new(UserRoleRel::UserId)
                            .integer()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(UserRoleRel::RoleId)
                            .integer()
                            .not_null()
                            .comment("角色ID"),
                    )
                    .col(
                        ColumnDef::new(UserRoleRel::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .to_owned(),
            )
            .await?;

        if_not_exists_create_unique_index(
            manager,
            UserRoleRel::Table,
            vec![UserRoleRel::UserId, UserRoleRel::RoleId],
        )
        .await?;

        // Sqlite 不支持外键
        if manager.get_database_backend() == DatabaseBackend::Sqlite {
            return Ok(());
        }

        if !manager
            .has_index(UserRoleRel::Table.to_string(), "fk_user_role_rel_user_id")
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_user_role_rel_user_id")
                        .from(UserRoleRel::Table, UserRoleRel::UserId)
                        .to(UserBase::Table, UserBase::Id)
                        .on_update(ForeignKeyAction::Cascade)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(UserRoleRel::Table.to_string(), "fk_user_role_rel_role_id")
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_user_role_rel_role_id")
                        .from(UserRoleRel::Table, UserRoleRel::RoleId)
                        .to(Role::Table, Role::Id)
                        .on_update(ForeignKeyAction::Cascade)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(UserRoleRel::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserRoleRel {
    #[sea_orm(iden = "t_user_role_rel")]
    Table,
    Id,
    UserId,
    RoleId,
    CreatedAt,
}

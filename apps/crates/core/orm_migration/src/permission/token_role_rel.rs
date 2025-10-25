//! 令牌角色关系表
//! Entity: [`entity::permission::TokenRoleRel`]
use crate::{permission::token::Token, user::role::Role, utils::if_not_exists_create_unique_index};

use sea_orm::{
    DatabaseBackend, DeriveIden, DeriveMigrationName, Iden,
    sea_query::{ColumnDef, Expr, ForeignKey, ForeignKeyAction, Table},
};
use sea_orm_migration::{DbErr, MigrationTrait, SchemaManager, async_trait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(TokenRoleRel::Table)
                    .comment("令牌角色关系表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TokenRoleRel::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(TokenRoleRel::TokenId)
                            .integer()
                            .not_null()
                            .comment("令牌ID"),
                    )
                    .col(
                        ColumnDef::new(TokenRoleRel::RoleId)
                            .integer()
                            .not_null()
                            .comment("角色ID"),
                    )
                    .col(
                        ColumnDef::new(TokenRoleRel::CreatedAt)
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
            TokenRoleRel::Table,
            vec![TokenRoleRel::TokenId, TokenRoleRel::RoleId],
        )
        .await?;

        // Sqlite 不支持外键
        if manager.get_database_backend() == DatabaseBackend::Sqlite {
            return Ok(());
        }

        if !manager
            .has_index(
                TokenRoleRel::Table.to_string(),
                "fk_perm_token_role_rel_token_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_perm_token_role_rel_token_id")
                        .from(TokenRoleRel::Table, TokenRoleRel::TokenId)
                        .to(Token::Table, Token::Id)
                        .on_update(ForeignKeyAction::Cascade)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        if !manager
            .has_index(
                TokenRoleRel::Table.to_string(),
                "fk_perm_token_role_rel_role_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_perm_token_role_rel_role_id")
                        .from(TokenRoleRel::Table, TokenRoleRel::RoleId)
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
            .drop_table(Table::drop().table(TokenRoleRel::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TokenRoleRel {
    #[sea_orm(iden = "t_perm_token_role_rel")]
    Table,
    Id,
    TokenId,
    RoleId,
    CreatedAt,
}

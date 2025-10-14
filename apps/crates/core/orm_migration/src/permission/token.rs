//! 令牌表
//! Entity: [`entity::permission::Token`]

use sea_orm::{
    DeriveIden, DeriveMigrationName, Iden,
    sea_query::{ColumnDef, Expr, Index, Table},
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
                    .table(Token::Table)
                    .comment("令牌表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Token::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("令牌ID"),
                    )
                    .col(
                        ColumnDef::new(Token::UserId)
                            .integer()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(Token::Token)
                            .string()
                            .string_len(50)
                            .unique_key()
                            .not_null()
                            .comment("令牌"),
                    )
                    .col(
                        ColumnDef::new(Token::Passphrase)
                            .string()
                            .string_len(20)
                            .not_null()
                            .comment("口令"),
                    )
                    .col(
                        ColumnDef::new(Token::Permission)
                            .string()
                            .string_len(20)
                            .not_null()
                            .comment("权限范围:GET,POST,PUT,DELETE"),
                    )
                    .col(
                        ColumnDef::new(Token::Expire)
                            .date_time()
                            .not_null()
                            .comment("授权到期时间"),
                    )
                    .col(
                        ColumnDef::new(Token::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(Token::Status)
                            .boolean()
                            .not_null()
                            .default(true)
                            .comment("状态(false:停用,true:正常)"),
                    )
                    .col(
                        ColumnDef::new(Token::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Token::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name(format!(
                        "idx_{}_{}",
                        Token::Table.to_string(),
                        Token::UserId.to_string()
                    ))
                    .table(Token::Table)
                    .col(Token::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name(format!(
                        "idx_{}_{}",
                        Token::Table.to_string(),
                        Token::Token.to_string()
                    ))
                    .table(Token::Table)
                    .col(Token::Token)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name(format!(
                        "idx_{}_{}",
                        Token::Table.to_string(),
                        Token::Passphrase.to_string()
                    ))
                    .table(Token::Table)
                    .col(Token::Passphrase)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Token::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Token {
    #[sea_orm(iden = "t_perm_token")]
    Table,
    Id,
    UserId,
    #[allow(clippy::enum_variant_names)]
    Token,
    Passphrase,
    Permission,
    Expire,
    Status,
    Desc,
    CreatedAt,
    UpdatedAt,
}

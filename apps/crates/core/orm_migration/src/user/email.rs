//! 用户邮箱表
//! Entity: [`entity::user::Email`]

use sea_orm::{
    DeriveIden, DeriveMigrationName,
    sea_query::{ColumnDef, Expr, Table},
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
                    .table(UserEmail::Table)
                    .comment("用户邮箱表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserEmail::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("邮箱ID"),
                    )
                    .col(
                        ColumnDef::new(UserEmail::UserId)
                            .integer()
                            .unique_key()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(UserEmail::Email)
                            .string()
                            .string_len(50)
                            .unique_key()
                            .not_null()
                            .comment("邮箱"),
                    )
                    .col(
                        ColumnDef::new(UserEmail::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(UserEmail::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(UserEmail::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(UserEmail::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserEmail {
    #[sea_orm(iden = "t_user_email")]
    Table,
    Id,
    UserId,
    Email,
    Desc,
    CreatedAt,
    UpdatedAt,
}

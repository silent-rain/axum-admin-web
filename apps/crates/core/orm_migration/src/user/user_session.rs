//! 用户session表
//! Entity: [`entity::user::UserSession`]

use sea_orm::{
    sea_query::{ColumnDef, Expr, Table},
    DeriveIden, DeriveMigrationName,
};
use sea_orm_migration::{async_trait, DbErr, MigrationTrait, SchemaManager};

use crate::utils::if_not_exists_create_index;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(UserSession::Table)
                    .comment("用户session表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserSession::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(UserSession::SessionId)
                            .string()
                            .string_len(22)
                            .not_null()
                            .comment("用户会话ID"),
                    )
                    .col(
                        ColumnDef::new(UserSession::ExpiryDate)
                            .timestamp_with_time_zone()
                            .not_null()
                            .comment("过期时间"),
                    )
                    .col(
                        ColumnDef::new(UserSession::Data)
                            .blob()
                            .not_null()
                            .comment("元数据"),
                    )
                    .col(
                        ColumnDef::new(UserSession::Status)
                            .boolean()
                            .not_null()
                            .default(true)
                            .comment("登录状态是否有效(false:无效,true:有效)"),
                    )
                    .col(
                        ColumnDef::new(UserSession::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(UserSession::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        if_not_exists_create_index(manager, UserSession::Table, vec![UserSession::Status]).await?;
        if_not_exists_create_index(manager, UserSession::Table, vec![UserSession::ExpiryDate])
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(UserSession::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserSession {
    #[sea_orm(iden = "t_user_session")]
    Table,
    Id,
    SessionId,
    ExpiryDate,
    Data,
    Status,
    CreatedAt,
    UpdatedAt,
}

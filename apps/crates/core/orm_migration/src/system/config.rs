//! 配置表
//! Entity: [`entity::system::SysConfig`]

use sea_orm::{
    DeriveIden, DeriveMigrationName,
    sea_query::{ColumnDef, Expr, Table},
};
use sea_orm_migration::{DbErr, MigrationTrait, SchemaManager, async_trait};

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
                    .table(Config::Table)
                    .comment("配置表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Config::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("配置ID"),
                    )
                    .col(
                        ColumnDef::new(Config::Pid)
                            .integer()
                            .null()
                            .default(0)
                            .comment("父节点ID"),
                    )
                    .col(
                        ColumnDef::new(Config::Name)
                            .string()
                            .string_len(64)
                            .not_null()
                            .comment("配置名称"),
                    )
                    .col(
                        ColumnDef::new(Config::Code)
                            .string()
                            .string_len(64)
                            .unique_key()
                            .not_null()
                            .comment("配置编码(英文)"),
                    )
                    .col(
                        ColumnDef::new(Config::Value)
                            .text()
                            .null()
                            .comment("配置值"),
                    )
                    .col(
                        ColumnDef::new(Config::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(Config::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("配置描述"),
                    )
                    .col(
                        ColumnDef::new(Config::Status)
                            .boolean()
                            .not_null()
                            .default(true)
                            .comment("状态(false:停用,true:正常)"),
                    )
                    .col(
                        ColumnDef::new(Config::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Config::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        if_not_exists_create_index(manager, Config::Table, vec![Config::Name]).await?;
        if_not_exists_create_index(manager, Config::Table, vec![Config::Pid]).await?;
        if_not_exists_create_index(manager, Config::Table, vec![Config::Code]).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Config::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Config {
    #[sea_orm(iden = "t_sys_config")]
    Table,
    Id,
    Pid,
    Name,
    Code,
    Value,
    Sort,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}

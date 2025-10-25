//! 字典维度表
//! Entity: [`entity::system::SysDictDimension`]
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
                    .table(DictDimension::Table)
                    .comment("字典维度表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DictDimension::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("字典维度ID"),
                    )
                    .col(
                        ColumnDef::new(DictDimension::Name)
                            .string()
                            .string_len(64)
                            .unique_key()
                            .not_null()
                            .comment("字典维度名称"),
                    )
                    .col(
                        ColumnDef::new(DictDimension::Code)
                            .string()
                            .string_len(64)
                            .unique_key()
                            .not_null()
                            .comment("字典维度编码"),
                    )
                    .col(
                        ColumnDef::new(DictDimension::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(DictDimension::Desc)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(DictDimension::Status)
                            .boolean()
                            .not_null()
                            .default(true)
                            .comment("状态(false:停用,true:正常)"),
                    )
                    .col(
                        ColumnDef::new(DictDimension::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(DictDimension::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        if_not_exists_create_index(manager, DictDimension::Table, vec![DictDimension::Name])
            .await?;
        if_not_exists_create_index(manager, DictDimension::Table, vec![DictDimension::Code])
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(DictDimension::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum DictDimension {
    #[sea_orm(iden = "t_sys_dict_dimension")]
    Table,
    Id,
    Name,
    Code,
    Sort,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}

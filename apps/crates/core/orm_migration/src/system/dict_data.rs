//! 字典数据表
//! Entity: [`entity::system::SysDictData`]
use crate::utils::if_not_exists_create_index;

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
                    .table(DictData::Table)
                    .comment("字典数据表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DictData::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("字典项ID"),
                    )
                    .col(
                        ColumnDef::new(DictData::DimId)
                            .integer()
                            .not_null()
                            .comment("字典维度ID"),
                    )
                    .col(
                        ColumnDef::new(DictData::Label)
                            .string()
                            .string_len(64)
                            .not_null()
                            .comment("字典项标签"),
                    )
                    .col(
                        ColumnDef::new(DictData::Value)
                            .text()
                            .not_null()
                            .comment("字典项值"),
                    )
                    .col(
                        ColumnDef::new(DictData::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(DictData::Desc)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(DictData::Status)
                            .boolean()
                            .not_null()
                            .default(true)
                            .comment("状态(false:停用,true:正常)"),
                    )
                    .col(
                        ColumnDef::new(DictData::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(DictData::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        if_not_exists_create_index(manager, DictData::Table, vec![DictData::DimId]).await?;
        if_not_exists_create_index(manager, DictData::Table, vec![DictData::Label]).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(DictData::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum DictData {
    #[sea_orm(iden = "t_sys_dict_data")]
    Table,
    Id,
    DimId,
    Label,
    Value,
    Sort,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}

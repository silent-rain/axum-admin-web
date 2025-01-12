//! 字典数据表
//! Entity: [`entity::system::SysDictData`]
use crate::utils::if_not_exists_create_index;

use sea_orm::{
    sea_query::{ColumnDef, Expr, Table},
    DeriveIden, DeriveMigrationName,
};
use sea_orm_migration::{async_trait, DbErr, MigrationTrait, SchemaManager};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(SysDictData::Table)
                    .comment("字典数据表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysDictData::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("字典项ID"),
                    )
                    .col(
                        ColumnDef::new(SysDictData::DimensionId)
                            .integer()
                            .not_null()
                            .comment("字典维度ID"),
                    )
                    .col(
                        ColumnDef::new(SysDictData::DimensionCode)
                            .string()
                            .string_len(64)
                            .not_null()
                            .comment("字典维度编码"),
                    )
                    .col(
                        ColumnDef::new(SysDictData::Label)
                            .string()
                            .string_len(64)
                            .not_null()
                            .comment("字典项标签"),
                    )
                    .col(
                        ColumnDef::new(SysDictData::Value)
                            .text()
                            .not_null()
                            .comment("字典项值"),
                    )
                    .col(
                        ColumnDef::new(SysDictData::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(SysDictData::Desc)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(SysDictData::Status)
                            .boolean()
                            .not_null()
                            .default(true)
                            .comment("状态(false:停用,true:正常)"),
                    )
                    .col(
                        ColumnDef::new(SysDictData::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(SysDictData::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        if_not_exists_create_index(manager, SysDictData::Table, vec![SysDictData::DimensionId])
            .await?;
        if_not_exists_create_index(
            manager,
            SysDictData::Table,
            vec![SysDictData::DimensionCode],
        )
        .await?;
        if_not_exists_create_index(manager, SysDictData::Table, vec![SysDictData::Label]).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(SysDictData::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SysDictData {
    #[sea_orm(iden = "t_sys_dict_data")]
    Table,
    Id,
    DimensionId,
    DimensionCode,
    Label,
    Value,
    Sort,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}

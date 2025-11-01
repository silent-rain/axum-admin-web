//! 部门表
//! Entity: [`entity::organization::Department`]

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
                    .table(Department::Table)
                    .comment("部门表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Department::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("部门ID"),
                    )
                    .col(
                        ColumnDef::new(Department::Pid)
                            .integer()
                            .null()
                            .default(0)
                            .comment("上级部门ID"),
                    )
                    .col(
                        ColumnDef::new(Department::Pids)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("所有上级部门ID, 用逗号分开"),
                    )
                    .col(
                        ColumnDef::new(Department::Name)
                            .string()
                            .string_len(20)
                            .not_null()
                            .comment("部门名称"),
                    )
                    .col(
                        ColumnDef::new(Department::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(Department::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(Department::Status)
                            .boolean()
                            .not_null()
                            .default(true)
                            .comment("状态(false:停用,true:正常)"),
                    )
                    .col(
                        ColumnDef::new(Department::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Department::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        // create index
        if_not_exists_create_index(manager, Department::Table, vec![Department::Pid]).await?;
        if_not_exists_create_index(manager, Department::Table, vec![Department::Name]).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Department::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Department {
    #[sea_orm(iden = "t_org_department")]
    Table,
    Id,
    Pid,
    Pids,
    Name,
    Sort,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}

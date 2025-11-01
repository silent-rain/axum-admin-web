//! 岗位表
//! Entity: [`entity::organization::Position`]

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
                    .table(Position::Table)
                    .comment("岗位表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Position::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("岗位ID"),
                    )
                    .col(
                        ColumnDef::new(Position::Name)
                            .string()
                            .string_len(100)
                            .unique_key()
                            .not_null()
                            .comment("岗位名称"),
                    )
                    .col(
                        ColumnDef::new(Position::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(Position::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(Position::DepartmentId)
                            .integer()
                            .null()
                            .default(0)
                            .comment("所属部门ID"),
                    )
                    .col(
                        ColumnDef::new(Position::Status)
                            .boolean()
                            .not_null()
                            .default(true)
                            .comment("状态(false:停用,true:正常)"),
                    )
                    .col(
                        ColumnDef::new(Position::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Position::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        // create index
        if_not_exists_create_index(manager, Position::Table, vec![Position::Name]).await?;
        if_not_exists_create_index(manager, Position::Table, vec![Position::DepartmentId]).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Position::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Position {
    #[sea_orm(iden = "t_org_position")]
    Table,
    Id,
    Name,
    Sort,
    Desc,
    DepartmentId,
    Status,
    CreatedAt,
    UpdatedAt,
}

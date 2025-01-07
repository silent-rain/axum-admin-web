//! 文件资源表
//! Entity: [`entity::system::SysFileResource`]

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
                    .table(SysFileResource::Table)
                    .comment("文件资源表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysFileResource::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("文件ID"),
                    )
                    .col(
                        ColumnDef::new(SysFileResource::Name)
                            .string()
                            .string_len(32)
                            .not_null()
                            .comment("文件名称"),
                    )
                    .col(
                        ColumnDef::new(SysFileResource::Hash)
                            .string()
                            .string_len(32)
                            .unique_key()
                            .not_null()
                            .comment("文件HASH值"),
                    )
                    .col(
                        ColumnDef::new(SysFileResource::Data)
                            .blob()
                            .not_null()
                            .comment("文件数据, Base64编码"),
                    )
                    .col(
                        ColumnDef::new(SysFileResource::Extension)
                            .string()
                            .string_len(10)
                            .not_null()
                            .comment("文件文件扩展名, 如svg, png"),
                    )
                    .col(
                        ColumnDef::new(SysFileResource::Size)
                            .integer()
                            .not_null()
                            .comment("文件文件大小，单位为字节"),
                    )
                    .col(
                        ColumnDef::new(SysFileResource::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(SysFileResource::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(SysFileResource::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SysFileResource {
    #[sea_orm(iden = "t_sys_file_resource")]
    Table,
    Id,
    Name,
    Hash,
    Data,
    Extension,
    Size,
    Desc,
    CreatedAt,
}

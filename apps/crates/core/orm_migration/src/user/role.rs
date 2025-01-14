//! 角色表
//! Entity: [`entity::user::Role`]

use sea_orm::{
    sea_query::{ColumnDef, Expr, Table},
    ConnectionTrait, DeriveIden, DeriveMigrationName,
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
                    .table(UserRole::Table)
                    .comment("角色表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserRole::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("角色ID"),
                    )
                    .col(
                        ColumnDef::new(UserRole::Name)
                            .string()
                            .string_len(20)
                            .unique_key()
                            .not_null()
                            .comment("角色名称"),
                    )
                    .col(
                        ColumnDef::new(UserRole::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(UserRole::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(UserRole::Status)
                            .boolean()
                            .not_null()
                            .default(true)
                            .comment("状态(false:停用,true:正常)"),
                    )
                    .col(
                        ColumnDef::new(UserRole::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(UserRole::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        // 预设数据
        db.execute_unprepared(
            "
            INSERT INTO
                t_user_role (`name`, `sort`, `status`)
            VALUES
                ('管理员', 1, 1),
                ('普通用户', 1, 1),
                ('开发工程师', 1, 1),
                ('设计师', 1, 1),
                ('客服人员', 1, 1);
            ",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(UserRole::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserRole {
    #[sea_orm(iden = "t_user_role")]
    Table,
    Id,
    Name,
    Sort,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}

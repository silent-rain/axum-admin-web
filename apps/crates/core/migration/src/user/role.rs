//! 角色表
//! Entity: [`entity::user::Role`]

use sea_orm::{
    ConnectionTrait, DeriveIden, DeriveMigrationName,
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
                    .table(Role::Table)
                    .comment("角色表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Role::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("角色ID"),
                    )
                    .col(
                        ColumnDef::new(Role::Name)
                            .string()
                            .string_len(20)
                            .unique_key()
                            .not_null()
                            .comment("角色名称"),
                    )
                    .col(
                        ColumnDef::new(Role::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(Role::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(Role::Status)
                            .boolean()
                            .not_null()
                            .default(true)
                            .comment("状态(false:停用,true:正常)"),
                    )
                    .col(
                        ColumnDef::new(Role::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Role::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        // 预设数据
        // let db = manager.get_connection();
        // Role::insert_many([
        //     role::ActiveModel {
        //         name: Set("管理员".to_string()),
        //         sort: Set(Some(1)),
        //         status: Set(true),
        //         ..Default::default()
        //     },
        //     role::ActiveModel {
        //         name: Set("普通用户".to_string()),
        //         sort: Set(Some(1)),
        //         status: Set(true),
        //         ..Default::default()
        //     },
        // ])
        // .exec(db)
        // .await?;

        // 预设数据
        {
            let db = manager.get_connection();

            // Use `execute_unprepared` if the SQL statement doesn't have value bindings
            db.execute_unprepared(
                r#"INSERT INTO `t_user_role` (`name`, `sort`, `status`) VALUES
                    ('管理员', 1, true),
                    ('普通用户', 1, true),
                    ('开发工程师', 1, true),
                    ('设计师', 1, true),
                    ('客服人员', 1, true)
                "#,
            )
            .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Role {
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

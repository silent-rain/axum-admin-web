//! 菜单表
//! Entity: [`entity::permission::Menu`]

use sea_orm::{
    DeriveIden, DeriveMigrationName, Iden,
    sea_query::{ColumnDef, Expr, Index, Table},
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
                    .table(Menu::Table)
                    .comment("菜单表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Menu::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("菜单ID"),
                    )
                    .col(
                        ColumnDef::new(Menu::Pid)
                            .integer()
                            .null()
                            .default(0)
                            .comment("父菜单ID"),
                    )
                    .col(
                        ColumnDef::new(Menu::Title)
                            .string()
                            .string_len(20)
                            .not_null()
                            .comment("菜单名称"),
                    )
                    .col(
                        ColumnDef::new(Menu::IconClass)
                            .string()
                            .string_len(20)
                            .null()
                            .default("")
                            .comment("Icon图标类"),
                    )
                    .col(
                        ColumnDef::new(Menu::MenuType)
                            .tiny_integer()
                            .not_null()
                            .default(0)
                            .comment("菜单类型(0:菜单,1:按钮)"),
                    )
                    .col(
                        ColumnDef::new(Menu::OpenMethod)
                            .tiny_integer()
                            .not_null()
                            .default(0)
                            .comment("打开方式(0:组件,1:内链,2:外链)"),
                    )
                    .col(
                        ColumnDef::new(Menu::Path)
                            .string()
                            .string_len(500)
                            .null()
                            .default("")
                            .comment("路由地址"),
                    )
                    .col(
                        ColumnDef::new(Menu::ComponentPath)
                            .string()
                            .string_len(500)
                            .null()
                            .default("")
                            .comment("组件路径"),
                    )
                    .col(
                        ColumnDef::new(Menu::RedirectTo)
                            .string()
                            .string_len(500)
                            .null()
                            .default("")
                            .comment("路由重定向"),
                    )
                    .col(
                        ColumnDef::new(Menu::Link)
                            .string()
                            .string_len(500)
                            .null()
                            .default("")
                            .comment("链接地址:站内链地址/站外链地址"),
                    )
                    .col(
                        ColumnDef::new(Menu::LinkTarget)
                            .string()
                            .string_len(20)
                            .null()
                            .default("_blank")
                            .comment("链接跳转方式,_blank/_self"),
                    )
                    .col(
                        ColumnDef::new(Menu::IsHidden)
                            .boolean()
                            .null()
                            .default(true)
                            .comment("是否隐藏(0:显示,1:隐藏)"),
                    )
                    .col(
                        ColumnDef::new(Menu::IsAlwaysShowRoot)
                            .boolean()
                            .null()
                            .default(true)
                            .comment("是否始终显示根菜单(0:隐藏,1:显示)"),
                    )
                    .col(
                        ColumnDef::new(Menu::Permission)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("权限标识"),
                    )
                    .col(
                        ColumnDef::new(Menu::Sort)
                            .integer()
                            .null()
                            .default(0)
                            .comment("排序"),
                    )
                    .col(
                        ColumnDef::new(Menu::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(Menu::Status)
                            .boolean()
                            .null()
                            .default(true)
                            .comment("状态(false:停用,true:正常)"),
                    )
                    .col(
                        ColumnDef::new(Menu::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Menu::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name(format!(
                        "idx_{}_{}",
                        Menu::Table.to_string(),
                        Menu::Pid.to_string()
                    ))
                    .table(Menu::Table)
                    .col(Menu::Pid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name(format!(
                        "idx_{}_{}",
                        Menu::Table.to_string(),
                        Menu::Title.to_string()
                    ))
                    .table(Menu::Table)
                    .col(Menu::Title)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Menu::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Menu {
    #[sea_orm(iden = "t_perm_menu")]
    Table,
    Id,
    Pid,
    Title,
    IconClass,
    #[allow(clippy::enum_variant_names)]
    MenuType,
    OpenMethod,
    Path,
    ComponentPath,
    RedirectTo,
    Link,
    LinkTarget,
    IsHidden,
    IsAlwaysShowRoot,
    Permission,
    Sort,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}

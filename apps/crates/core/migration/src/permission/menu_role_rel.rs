//! 菜单角色关系表
//! Entity: [`entity::permission::MenuRoleRel`]

use sea_orm::{
    DeriveIden, DeriveMigrationName, Iden,
    sea_query::{ColumnDef, Expr, ForeignKey, ForeignKeyAction, Table},
};
use sea_orm_migration::{DbErr, MigrationTrait, SchemaManager, async_trait};

use crate::{permission::menu::Menu, user::role::Role, utils::if_not_exists_create_unique_index};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(MenuRoleRel::Table)
                    .comment("菜单角色关系表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MenuRoleRel::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(MenuRoleRel::MenuId)
                            .integer()
                            .not_null()
                            .comment("菜单ID"),
                    )
                    .col(
                        ColumnDef::new(MenuRoleRel::RoleId)
                            .integer()
                            .not_null()
                            .comment("角色ID"),
                    )
                    .col(
                        ColumnDef::new(MenuRoleRel::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(format!(
                                "fk_{}_{}",
                                MenuRoleRel::Table.to_string(),
                                MenuRoleRel::MenuId.to_string()
                            ))
                            .from_col(MenuRoleRel::MenuId)
                            .to(Menu::Table, Menu::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(format!(
                                "fk_{}_{}",
                                MenuRoleRel::Table.to_string(),
                                MenuRoleRel::RoleId.to_string()
                            ))
                            .from_col(MenuRoleRel::RoleId)
                            .to(Role::Table, Role::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // create unique index
        if_not_exists_create_unique_index(
            manager,
            MenuRoleRel::Table,
            vec![MenuRoleRel::MenuId, MenuRoleRel::RoleId],
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(MenuRoleRel::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum MenuRoleRel {
    #[sea_orm(iden = "t_perm_menu_role_rel")]
    Table,
    Id,
    MenuId,
    RoleId,
    CreatedAt,
}

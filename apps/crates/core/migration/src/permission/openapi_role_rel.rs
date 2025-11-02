//! OpenApi接口角色关系表
//! Entity: [`entity::permission::OpenapiRoleRel`]

use sea_orm::{
    DeriveIden, DeriveMigrationName, Iden,
    sea_query::{ColumnDef, Expr, ForeignKey, ForeignKeyAction, Table},
};
use sea_orm_migration::{DbErr, MigrationTrait, SchemaManager, async_trait};

use crate::{
    permission::openapi::Openapi, user::role::Role, utils::if_not_exists_create_unique_index,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(OpenapiRoleRel::Table)
                    .comment("OpenApi接口角色关系表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OpenapiRoleRel::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(OpenapiRoleRel::OpenapiId)
                            .integer()
                            .not_null()
                            .comment("接口ID"),
                    )
                    .col(
                        ColumnDef::new(OpenapiRoleRel::RoleId)
                            .integer()
                            .not_null()
                            .comment("角色ID"),
                    )
                    .col(
                        ColumnDef::new(OpenapiRoleRel::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(format!(
                                "fk_{}_{}",
                                OpenapiRoleRel::Table.to_string(),
                                OpenapiRoleRel::OpenapiId.to_string()
                            ))
                            .from_col(OpenapiRoleRel::OpenapiId)
                            .to(Openapi::Table, Openapi::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(format!(
                                "fk_{}_{}",
                                OpenapiRoleRel::Table.to_string(),
                                OpenapiRoleRel::RoleId.to_string()
                            ))
                            .from_col(OpenapiRoleRel::RoleId)
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
            OpenapiRoleRel::Table,
            vec![OpenapiRoleRel::OpenapiId, OpenapiRoleRel::RoleId],
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(OpenapiRoleRel::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum OpenapiRoleRel {
    #[sea_orm(iden = "t_perm_openapi_role_rel")]
    Table,
    Id,
    OpenapiId,
    RoleId,
    CreatedAt,
}

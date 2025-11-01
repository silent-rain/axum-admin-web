//! 部门角色关系表
//! Entity: [`entity::organization::DepartmentRoleRel`]
use crate::{
    organization::department::Department, user::role::Role,
    utils::if_not_exists_create_unique_index,
};

use sea_orm::{
    DeriveIden, DeriveMigrationName, Iden,
    sea_query::{ColumnDef, Expr, ForeignKey, ForeignKeyAction, Table},
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
                    .table(DepartmentRoleRel::Table)
                    .comment("部门角色关系表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DepartmentRoleRel::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(DepartmentRoleRel::DepartmentId)
                            .integer()
                            .not_null()
                            .comment("部门ID"),
                    )
                    .col(
                        ColumnDef::new(DepartmentRoleRel::RoleId)
                            .integer()
                            .not_null()
                            .comment("角色ID"),
                    )
                    .col(
                        ColumnDef::new(DepartmentRoleRel::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(format!(
                                "fk_{}_{}",
                                DepartmentRoleRel::Table.to_string(),
                                DepartmentRoleRel::DepartmentId.to_string()
                            ))
                            .from_col(DepartmentRoleRel::DepartmentId)
                            .to(Department::Table, Department::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(format!(
                                "fk_{}_{}",
                                DepartmentRoleRel::Table.to_string(),
                                DepartmentRoleRel::RoleId.to_string()
                            ))
                            .from_col(DepartmentRoleRel::RoleId)
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
            DepartmentRoleRel::Table,
            vec![DepartmentRoleRel::DepartmentId, DepartmentRoleRel::RoleId],
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(DepartmentRoleRel::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum DepartmentRoleRel {
    #[sea_orm(iden = "t_org_department_role_rel")]
    Table,
    Id,
    DepartmentId,
    RoleId,
    CreatedAt,
}

//! 任务调度状态日志表
//! Entity: [`entity::schedule::ScheduleStatusLog`]

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
                    .table(ScheduleStatusLog::Table)
                    .comment("任务调度状态日志表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ScheduleStatusLog::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("状态日志ID"),
                    )
                    .col(
                        ColumnDef::new(ScheduleStatusLog::JobId)
                            .integer()
                            .not_null()
                            .comment("任务ID"),
                    )
                    .col(
                        ColumnDef::new(ScheduleStatusLog::Uuid)
                            .string()
                            .string_len(50)
                            .comment("任务调度ID"),
                    )
                    .col(
                        ColumnDef::new(ScheduleStatusLog::Error)
                            .text()
                            .comment("失败信息"),
                    )
                    .col(
                        ColumnDef::new(ScheduleStatusLog::Cost)
                            .small_integer()
                            .not_null()
                            .comment("耗时,毫秒"),
                    )
                    .col(
                        ColumnDef::new(ScheduleStatusLog::Status)
                            .tiny_integer()
                            .not_null()
                            .default(0)
                            .comment("任务状态(0:开始,1:完成,2:停止,3:移除)"),
                    )
                    .col(
                        ColumnDef::new(ScheduleStatusLog::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(ScheduleStatusLog::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        // create index
        if_not_exists_create_index(
            manager,
            ScheduleStatusLog::Table,
            vec![ScheduleStatusLog::JobId],
        )
        .await?;

        if_not_exists_create_index(
            manager,
            ScheduleStatusLog::Table,
            vec![ScheduleStatusLog::Uuid],
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(ScheduleStatusLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ScheduleStatusLog {
    #[sea_orm(iden = "t_schedule_status_log")]
    Table,
    Id,
    JobId,
    Uuid,
    Error,
    Cost,
    Status,
    CreatedAt,
    UpdatedAt,
}

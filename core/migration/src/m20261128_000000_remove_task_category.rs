use sea_orm::DbBackend;
use sea_orm_migration::prelude::*;

use crate::m20261127_134554_create_task_category_table::TaskCategory;
use crate::m20261127_134557_add_category_to_task;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261128_000000_remove_task_category"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.get_database_backend() == DbBackend::Postgres {
            manager
                .get_connection()
                .execute_unprepared(
                    r#"ALTER TABLE "task" DROP CONSTRAINT IF EXISTS "fk_task_category_identifier""#,
                )
                .await?;
        }

        manager
            .alter_table(
                Table::alter()
                    .table(Task::Table)
                    .drop_column(TaskCol::CategoryIdentifier)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(TaskCategory::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        crate::m20261127_134554_create_task_category_table::Migration
            .up(manager)
            .await?;
        m20261127_134557_add_category_to_task::Migration
            .up(manager)
            .await
    }
}

#[derive(DeriveIden)]
enum Task {
    Table,
}

#[derive(DeriveIden)]
enum TaskCol {
    CategoryIdentifier,
}

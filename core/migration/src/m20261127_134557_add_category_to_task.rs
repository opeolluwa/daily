use sea_orm_migration::prelude::*;

use crate::m20261127_134554_create_task_category_table::TaskCategory;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261127_134557_add_category_to_task"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();

        let column = ColumnDef::new(TaskCol::CategoryIdentifier)
            .string()
            .null()
            .to_owned();

        if backend == sea_orm::DbBackend::MySql {
            manager
                .alter_table(
                    Table::alter()
                        .table(Task::Table)
                        .add_column(column)
                        .to_owned(),
                )
                .await?;
        } else {
            manager
                .alter_table(
                    Table::alter()
                        .table(Task::Table)
                        .add_column_if_not_exists(column)
                        .to_owned(),
                )
                .await?;
        }

        if backend == sea_orm::DbBackend::Postgres {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_task_category_identifier")
                        .from(Task::Table, TaskCol::CategoryIdentifier)
                        .to(TaskCategory::Table, TaskCategory::Identifier)
                        .on_delete(ForeignKeyAction::SetNull)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();

        if backend == sea_orm::DbBackend::Postgres {
            manager
                .drop_foreign_key(
                    ForeignKey::drop()
                        .name("fk_task_category_identifier")
                        .table(Task::Table)
                        .to_owned(),
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

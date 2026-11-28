use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261127_234037_create_task_subtasks"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TaskSubtask::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TaskSubtask::Identifier)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(string(TaskSubtask::TaskIdentifier))
                    .col(string(TaskSubtask::Title))
                    .col(boolean(TaskSubtask::Done).default(false))
                    .col(integer(TaskSubtask::SortOrder).default(0))
                    .col(string_null(TaskSubtask::WorkspaceIdentifier))
                    .col(timestamp_with_time_zone(TaskSubtask::CreatedAt))
                    .col(timestamp_with_time_zone(TaskSubtask::UpdatedAt))
                    .col(timestamp_with_time_zone_null(TaskSubtask::DeletedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_subtask_task")
                            .from(TaskSubtask::Table, TaskSubtask::TaskIdentifier)
                            .to(Task::Table, Task::Identifier)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_subtask_workspace")
                            .from(TaskSubtask::Table, TaskSubtask::WorkspaceIdentifier)
                            .to(Workspaces::Table, Workspaces::Identifier)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_task_subtask_task")
                    .table(TaskSubtask::Table)
                    .col(TaskSubtask::TaskIdentifier)
                    .col(TaskSubtask::DeletedAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_task_subtask_task")
                    .table(TaskSubtask::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(TaskSubtask::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TaskSubtask {
    Table,
    Identifier,
    TaskIdentifier,
    Title,
    Done,
    SortOrder,
    WorkspaceIdentifier,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(DeriveIden)]
enum Task {
    Table,
    Identifier,
}

#[derive(DeriveIden)]
enum Workspaces {
    Table,
    Identifier,
}

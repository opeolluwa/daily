use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261127_134554_create_task_category_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TaskCategory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TaskCategory::Identifier)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(string(TaskCategory::Name))
                    .col(string(TaskCategory::Color))
                    .col(string_null(TaskCategory::WorkspaceIdentifier))
                    .col(timestamp_with_time_zone(TaskCategory::CreatedAt))
                    .col(timestamp_with_time_zone(TaskCategory::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_category_workspace")
                            .from(TaskCategory::Table, TaskCategory::WorkspaceIdentifier)
                            .to(Workspaces::Table, Workspaces::Identifier)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TaskCategory::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TaskCategory {
    Table,
    Identifier,
    Name,
    Color,
    WorkspaceIdentifier,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Workspaces {
    Table,
    Identifier,
}

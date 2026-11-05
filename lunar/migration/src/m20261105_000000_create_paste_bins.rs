use sea_orm_migration::{
    prelude::*,
    schema::{boolean, pk_uuid, string_null, text, timestamp_with_time_zone, uuid_null},
    sea_orm::DbBackend,
};

use crate::m20260224_214545_create_workspaces::Workspaces;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_backend = manager.get_database_backend();
        let db_connection = manager.get_connection();

        if db_backend == DbBackend::Postgres {
            db_connection
                .execute_unprepared(
                    "ALTER TYPE \"item_type\" ADD VALUE IF NOT EXISTS 'paste_bin';",
                )
                .await?;
        }

        if db_backend == DbBackend::MySql {
            db_connection
                .execute_unprepared(
                    "ALTER TABLE `recycle_bin` MODIFY COLUMN `item_type` ENUM('todo', 'note', 'reminder', 'snippet', 'bookmark', 'workspace', 'paste_bin') NOT NULL;",
                )
                .await?;
        }

        manager
            .create_table(
                Table::create()
                    .table(PasteBins::Table)
                    .if_not_exists()
                    .col(pk_uuid(PasteBins::Identifier))
                    .col(string_null(PasteBins::Title))
                    .col(text(PasteBins::Content))
                    .col(boolean(PasteBins::IsPinned).default(false))
                    .col(timestamp_with_time_zone(PasteBins::CreatedAt))
                    .col(timestamp_with_time_zone(PasteBins::UpdatedAt))
                    .col(uuid_null(PasteBins::WorkspaceIdentifier))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_paste_bins_workspace_identifier")
                            .from(PasteBins::Table, PasteBins::WorkspaceIdentifier)
                            .to(Workspaces::Table, Workspaces::Identifier)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_paste_bins_workspace_identifier")
                    .table(PasteBins::Table)
                    .col(PasteBins::WorkspaceIdentifier)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_paste_bins_workspace_identifier")
                    .table(PasteBins::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(PasteBins::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PasteBins {
    Table,
    Identifier,
    Title,
    Content,
    IsPinned,
    CreatedAt,
    UpdatedAt,
    WorkspaceIdentifier,
}

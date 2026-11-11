use sea_orm_migration::{
    prelude::*,
    schema::{
        json_null, pk_uuid, string, string_null, text, timestamp_with_time_zone, uuid, uuid_null,
    },
};

use crate::m20260224_214545_create_workspaces::Workspaces;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Whiteboards::Table)
                    .if_not_exists()
                    .col(pk_uuid(Whiteboards::Identifier))
                    .col(string_null(Whiteboards::Title))
                    .col(json_null(Whiteboards::Payload))
                    .col(timestamp_with_time_zone(Whiteboards::CreatedAt))
                    .col(timestamp_with_time_zone(Whiteboards::UpdatedAt))
                    .col(uuid_null(Whiteboards::WorkspaceIdentifier))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_whiteboards_workspace_identifier")
                            .from(Whiteboards::Table, Whiteboards::WorkspaceIdentifier)
                            .to(Workspaces::Table, Workspaces::Identifier)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_whiteboards_workspace_identifier")
                    .table(Whiteboards::Table)
                    .col(Whiteboards::WorkspaceIdentifier)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(WhiteboardFiles::Table)
                    .if_not_exists()
                    .col(pk_uuid(WhiteboardFiles::Identifier))
                    .col(uuid(WhiteboardFiles::WhiteboardIdentifier))
                    .col(string(WhiteboardFiles::MimeType))
                    .col(text(WhiteboardFiles::Data))
                    .col(timestamp_with_time_zone(WhiteboardFiles::CreatedAt))
                    .col(timestamp_with_time_zone(WhiteboardFiles::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_whiteboard_files_whiteboard_identifier")
                            .from(
                                WhiteboardFiles::Table,
                                WhiteboardFiles::WhiteboardIdentifier,
                            )
                            .to(Whiteboards::Table, Whiteboards::Identifier)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_whiteboard_files_whiteboard_identifier")
                    .table(WhiteboardFiles::Table)
                    .col(WhiteboardFiles::WhiteboardIdentifier)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_whiteboard_files_whiteboard_identifier")
                    .table(WhiteboardFiles::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(WhiteboardFiles::Table).to_owned())
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_whiteboards_workspace_identifier")
                    .table(Whiteboards::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Whiteboards::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Whiteboards {
    Table,
    Identifier,
    Title,
    Payload,
    CreatedAt,
    UpdatedAt,
    WorkspaceIdentifier,
}

#[derive(DeriveIden)]
enum WhiteboardFiles {
    Table,
    Identifier,
    WhiteboardIdentifier,
    MimeType,
    Data,
    CreatedAt,
    UpdatedAt,
}

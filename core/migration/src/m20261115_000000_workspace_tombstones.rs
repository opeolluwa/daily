use sea_orm_migration::{
    prelude::*,
    sea_orm::DbBackend,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = manager.get_database_backend();

        match backend {
            DbBackend::Sqlite => {
                if !manager.has_column("workspaces", "deleted_at").await? {
                    db.execute_unprepared(
                        "ALTER TABLE \"workspaces\" ADD COLUMN deleted_at TEXT;",
                    )
                    .await?;
                }
            }
            DbBackend::Postgres => {
                db.execute_unprepared(
                    "ALTER TABLE \"workspaces\" ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;",
                )
                .await?;
            }
            DbBackend::MySql => {
                if !manager.has_column("workspaces", "deleted_at").await? {
                    db.execute_unprepared(
                        "ALTER TABLE `workspaces` ADD COLUMN `deleted_at` DATETIME(6) NULL;",
                    )
                    .await?;
                }
            }
            _ => {}
        }

        manager
            .create_index(
                Index::create()
                    .name("idx_workspaces_deleted_at")
                    .table(Workspaces::Table)
                    .col(Workspaces::DeletedAt)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_workspaces_deleted_at")
                    .table(Workspaces::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Workspaces::Table)
                    .drop_column(Workspaces::DeletedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Workspaces {
    Table,
    DeletedAt,
}

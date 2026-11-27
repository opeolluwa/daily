use sea_orm_migration::{prelude::*, sea_orm::DbBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

const SYNCABLE_TABLES: &[&str] = &[
    "notes",
    "task",
    "bookmark",
    "snippets",
    "reminder",
    "paste_bins",
    "whiteboards",
    "workspace_profiles",
    "workspaces",
];

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.get_database_backend() != DbBackend::Postgres {
            return Ok(());
        }

        let db = manager.get_connection();

        manager
            .create_table(
                Table::create()
                    .table(SyncLog::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SyncLog::Seq)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SyncLog::WorkspaceIdentifier).uuid().null())
                    .col(ColumnDef::new(SyncLog::TableName).text().not_null())
                    .col(ColumnDef::new(SyncLog::RecordIdentifier).uuid().null())
                    .col(ColumnDef::new(SyncLog::Operation).text().not_null())
                    .col(ColumnDef::new(SyncLog::Payload).json_binary().null())
                    .col(
                        ColumnDef::new(SyncLog::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_sync_log_workspace_seq")
                    .table(SyncLog::Table)
                    .col(SyncLog::WorkspaceIdentifier)
                    .col(SyncLog::Seq)
                    .to_owned(),
            )
            .await?;

        // The kernel migration also installs queue triggers for the local
        // SQLite databases. The server does not consume that queue.
        for table in SYNCABLE_TABLES {
            db.execute_unprepared(&format!(
                "DROP TRIGGER IF EXISTS {table}_sync_trigger ON \"{table}\";"
            ))
            .await?;
        }
        db.execute_unprepared("DROP FUNCTION IF EXISTS enqueue_sync;")
            .await?;

        db.execute_unprepared(
            r#"
CREATE OR REPLACE FUNCTION log_sync_change()
RETURNS TRIGGER AS $$
DECLARE
    rec_id uuid;
    ws_id uuid;
BEGIN
    IF TG_OP = 'DELETE' THEN
        rec_id := OLD.identifier;
        ws_id := COALESCE((to_jsonb(OLD) ->> 'workspace_identifier')::uuid, OLD.identifier);
    ELSE
        rec_id := NEW.identifier;
        ws_id := COALESCE((to_jsonb(NEW) ->> 'workspace_identifier')::uuid, NEW.identifier);
    END IF;

    INSERT INTO sync_log(workspace_identifier, table_name, record_identifier, operation, payload)
    VALUES (
        ws_id,
        TG_TABLE_NAME,
        rec_id,
        TG_OP,
        CASE WHEN TG_OP = 'DELETE' THEN to_jsonb(OLD) ELSE to_jsonb(NEW) END
    );

    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;
"#,
        )
        .await?;

        for table in SYNCABLE_TABLES {
            db.execute_unprepared(&format!(
                "DROP TRIGGER IF EXISTS {table}_sync_log_trigger ON \"{table}\";"
            ))
            .await?;
            db.execute_unprepared(&format!(
                "CREATE TRIGGER {table}_sync_log_trigger
                 AFTER INSERT OR UPDATE OR DELETE ON \"{table}\"
                 FOR EACH ROW EXECUTE FUNCTION log_sync_change();"
            ))
            .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.get_database_backend() != DbBackend::Postgres {
            return Ok(());
        }

        let db = manager.get_connection();

        for table in SYNCABLE_TABLES {
            db.execute_unprepared(&format!(
                "DROP TRIGGER IF EXISTS {table}_sync_log_trigger ON \"{table}\";"
            ))
            .await?;
        }
        db.execute_unprepared("DROP FUNCTION IF EXISTS log_sync_change;")
            .await?;

        manager
            .drop_table(Table::drop().table(SyncLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SyncLog {
    Table,
    Seq,
    WorkspaceIdentifier,
    TableName,
    RecordIdentifier,
    Operation,
    Payload,
    CreatedAt,
}

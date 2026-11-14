use sea_orm_migration::{
    prelude::*,
    schema::{pk_uuid, string, text, timestamp_with_time_zone, uuid_null},
    sea_orm::DbBackend,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

const SYNCABLE_TABLES: &[(&str, bool)] = &[
    ("notes", false),
    ("task", false),
    ("bookmark", false),
    ("snippets", false),
    ("reminder", false),
    ("paste_bins", false),
    ("whiteboards", false),
    ("workspace_profiles", false),
    ("workspaces", true),
];

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = manager.get_database_backend();

        // 1. Soft-delete columns on every syncable table.
        for (table, _) in SYNCABLE_TABLES {
            if *table == "workspaces" {
                continue;
            }
            match backend {
                DbBackend::Sqlite => {
                    if !manager.has_column(table, "deleted_at").await? {
                        db.execute_unprepared(&format!(
                            "ALTER TABLE \"{table}\" ADD COLUMN deleted_at TEXT;"
                        ))
                        .await?;
                    }
                }
                DbBackend::Postgres => {
                    db.execute_unprepared(&format!(
                        "ALTER TABLE \"{table}\" ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;"
                    ))
                    .await?;
                }
                DbBackend::MySql => {
                    if !manager.has_column(table, "deleted_at").await? {
                        db.execute_unprepared(&format!(
                            "ALTER TABLE `{table}` ADD COLUMN deleted_at DATETIME(6) NULL;"
                        ))
                        .await?;
                    }
                }
                _ => {}
            }
        }

        // 2. Local mutation queue.
        manager
            .create_table(
                Table::create()
                    .table(SyncQueue::Table)
                    .if_not_exists()
                    .col(pk_uuid(SyncQueue::Identifier))
                    .col(string(SyncQueue::TableName))
                    .col(string(SyncQueue::RecordIdentifier))
                    .col(string(SyncQueue::Operation))
                    .col(uuid_null(SyncQueue::WorkspaceIdentifier))
                    .col(
                        timestamp_with_time_zone(SyncQueue::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(SyncQueue::Attempts)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(text(SyncQueue::LastError).null())
                    .col(string(SyncQueue::DeviceId).null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_sync_queue_workspace")
                    .table(SyncQueue::Table)
                    .col(SyncQueue::WorkspaceIdentifier)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_sync_queue_record")
                    .table(SyncQueue::Table)
                    .col(SyncQueue::TableName)
                    .col(SyncQueue::RecordIdentifier)
                    .to_owned(),
            )
            .await?;

        // 3. Per-workspace cursor state.
        manager
            .create_table(
                Table::create()
                    .table(SyncState::Table)
                    .if_not_exists()
                    .col(pk_uuid(SyncState::WorkspaceIdentifier))
                    .col(
                        ColumnDef::new(SyncState::LastCursor)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(timestamp_with_time_zone(SyncState::LastSyncedAt).null())
                    .col(text(SyncState::LastError).null())
                    .to_owned(),
            )
            .await?;

        // 4. Scope-local metadata (used to suppress trigger echo while applying).
        manager
            .create_table(
                Table::create()
                    .table(SyncMeta::Table)
                    .if_not_exists()
                    .col(string(SyncMeta::Key).primary_key())
                    .col(text(SyncMeta::Value))
                    .to_owned(),
            )
            .await?;

        // 5. Install per-row enqueue triggers.
        match backend {
            DbBackend::Sqlite => self.create_sqlite_triggers(db).await?,
            DbBackend::Postgres => self.create_postgres_triggers(db).await?,
            DbBackend::MySql => self.create_mysql_triggers(db).await?,
            _ => {}
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = manager.get_database_backend();

        match backend {
            DbBackend::Sqlite => {
                for (table, _) in SYNCABLE_TABLES {
                    db.execute_unprepared(&format!(
                        "DROP TRIGGER IF EXISTS {table}_sync_insert;
                         DROP TRIGGER IF EXISTS {table}_sync_update;
                         DROP TRIGGER IF EXISTS {table}_sync_delete;"
                    ))
                    .await?;
                }
                db.execute_unprepared("DROP TABLE IF EXISTS sync_meta;")
                    .await?;
                db.execute_unprepared("DROP TABLE IF EXISTS sync_state;")
                    .await?;
                db.execute_unprepared("DROP TABLE IF EXISTS sync_queue;")
                    .await?;
            }
            DbBackend::Postgres => {
                for (table, _) in SYNCABLE_TABLES {
                    db.execute_unprepared(&format!(
                        "DROP TRIGGER IF EXISTS {table}_sync_trigger ON \"{table}\";"
                    ))
                    .await?;
                }
                db.execute_unprepared("DROP FUNCTION IF EXISTS enqueue_sync;")
                    .await?;
                db.execute_unprepared("DROP TABLE IF EXISTS sync_meta;")
                    .await?;
                db.execute_unprepared("DROP TABLE IF EXISTS sync_state;")
                    .await?;
                db.execute_unprepared("DROP TABLE IF EXISTS sync_queue;")
                    .await?;
            }
            DbBackend::MySql => {
                for (table, _) in SYNCABLE_TABLES {
                    db.execute_unprepared(&format!(
                        "DROP TRIGGER IF EXISTS {table}_sync_insert;
                         DROP TRIGGER IF EXISTS {table}_sync_update;
                         DROP TRIGGER IF EXISTS {table}_sync_delete;"
                    ))
                    .await?;
                }
                db.execute_unprepared("DROP TABLE IF EXISTS sync_meta;")
                    .await?;
                db.execute_unprepared("DROP TABLE IF EXISTS sync_state;")
                    .await?;
                db.execute_unprepared("DROP TABLE IF EXISTS sync_queue;")
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}

impl Migration {
    async fn create_sqlite_triggers(&self, db: &SchemaManagerConnection<'_>) -> Result<(), DbErr> {
        for (table, is_workspace) in SYNCABLE_TABLES {
            let insert_ws = if *is_workspace {
                "NEW.identifier"
            } else {
                "NEW.workspace_identifier"
            };
            let update_ws = if *is_workspace {
                "NEW.identifier"
            } else {
                "NEW.workspace_identifier"
            };
            let delete_ws = if *is_workspace {
                "OLD.identifier"
            } else {
                "OLD.workspace_identifier"
            };

            let sql = format!(
                r#"
CREATE TRIGGER IF NOT EXISTS {table}_sync_insert
AFTER INSERT ON "{table}"
FOR EACH ROW
BEGIN
    INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
    SELECT randomblob(16), '{table}', lower(hex(NEW.identifier)), 'INSERT', {insert_ws}, datetime('now'), 0
    WHERE (SELECT value FROM sync_meta WHERE key = 'applying') IS NULL;
END;

CREATE TRIGGER IF NOT EXISTS {table}_sync_update
AFTER UPDATE ON "{table}"
FOR EACH ROW
BEGIN
    INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
    SELECT randomblob(16), '{table}', lower(hex(NEW.identifier)), 'UPDATE', {update_ws}, datetime('now'), 0
    WHERE (SELECT value FROM sync_meta WHERE key = 'applying') IS NULL;
END;

CREATE TRIGGER IF NOT EXISTS {table}_sync_delete
AFTER DELETE ON "{table}"
FOR EACH ROW
BEGIN
    INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
    SELECT randomblob(16), '{table}', lower(hex(OLD.identifier)), 'DELETE', {delete_ws}, datetime('now'), 0
    WHERE (SELECT value FROM sync_meta WHERE key = 'applying') IS NULL;
END;
"#
            );

            db.execute_unprepared(&sql).await?;
        }

        Ok(())
    }

    async fn create_postgres_triggers(
        &self,
        db: &SchemaManagerConnection<'_>,
    ) -> Result<(), DbErr> {
        for (table, _) in SYNCABLE_TABLES {
            db.execute_unprepared(&format!(
                "DROP TRIGGER IF EXISTS {table}_sync_trigger ON \"{table}\";"
            ))
            .await?;
        }

        db.execute_unprepared(
            r#"
CREATE OR REPLACE FUNCTION enqueue_sync()
RETURNS TRIGGER AS $$
DECLARE
    applying text;
    rec_id uuid;
    ws_id uuid;
BEGIN
    SELECT value INTO applying FROM sync_meta WHERE key = 'applying';
    IF applying IS NOT NULL AND applying <> '' THEN
        RETURN COALESCE(NEW, OLD);
    END IF;

    IF TG_OP = 'DELETE' THEN
        rec_id := OLD.identifier;
        ws_id := COALESCE((to_jsonb(OLD) ->> 'workspace_identifier')::uuid, OLD.identifier);
    ELSE
        rec_id := NEW.identifier;
        ws_id := COALESCE((to_jsonb(NEW) ->> 'workspace_identifier')::uuid, NEW.identifier);
    END IF;

    INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
    VALUES (gen_random_uuid(), TG_TABLE_NAME, rec_id::text, TG_OP, ws_id, NOW(), 0);

    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;
"#,
        )
        .await?;

        for (table, _) in SYNCABLE_TABLES {
            db.execute_unprepared(&format!(
                "CREATE TRIGGER {table}_sync_trigger
                 AFTER INSERT OR UPDATE OR DELETE ON \"{table}\"
                 FOR EACH ROW EXECUTE FUNCTION enqueue_sync();"
            ))
            .await?;
        }

        Ok(())
    }

    async fn create_mysql_triggers(&self, db: &SchemaManagerConnection<'_>) -> Result<(), DbErr> {
        for (table, is_workspace) in SYNCABLE_TABLES {
            let insert_ws = if *is_workspace {
                "NEW.identifier"
            } else {
                "NEW.workspace_identifier"
            };
            let update_ws = if *is_workspace {
                "NEW.identifier"
            } else {
                "NEW.workspace_identifier"
            };
            let delete_ws = if *is_workspace {
                "OLD.identifier"
            } else {
                "OLD.workspace_identifier"
            };

            let sql = format!(
                r#"
DROP TRIGGER IF EXISTS {table}_sync_insert;
CREATE TRIGGER {table}_sync_insert
AFTER INSERT ON `{table}` FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
SELECT UUID_TO_BIN(UUID()), '{table}', BIN_TO_UUID(NEW.identifier), 'INSERT', {insert_ws}, NOW(6), 0
FROM sync_meta WHERE `key` = 'applying' AND value IS NULL;

DROP TRIGGER IF EXISTS {table}_sync_update;
CREATE TRIGGER {table}_sync_update
AFTER UPDATE ON `{table}` FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
SELECT UUID_TO_BIN(UUID()), '{table}', BIN_TO_UUID(NEW.identifier), 'UPDATE', {update_ws}, NOW(6), 0
FROM sync_meta WHERE `key` = 'applying' AND value IS NULL;

DROP TRIGGER IF EXISTS {table}_sync_delete;
CREATE TRIGGER {table}_sync_delete
AFTER DELETE ON `{table}` FOR EACH ROW
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
SELECT UUID_TO_BIN(UUID()), '{table}', BIN_TO_UUID(OLD.identifier), 'DELETE', {delete_ws}, NOW(6), 0
FROM sync_meta WHERE `key` = 'applying' AND value IS NULL;
"#
            );

            db.execute_unprepared(&sql).await?;
        }

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum SyncQueue {
    Table,
    Identifier,
    TableName,
    RecordIdentifier,
    Operation,
    WorkspaceIdentifier,
    CreatedAt,
    Attempts,
    LastError,
    DeviceId,
}

#[derive(DeriveIden)]
pub enum SyncState {
    Table,
    WorkspaceIdentifier,
    LastCursor,
    LastSyncedAt,
    LastError,
}

#[derive(DeriveIden)]
pub enum SyncMeta {
    Table,
    Key,
    Value,
}

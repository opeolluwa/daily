use sea_orm::DbBackend;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const SYNCABLE_TABLES: &[(&str, bool)] = &[
    ("workspaces", true),
    ("workspace_profiles", false),
    ("notes", false),
    ("task", false),
    ("bookmark", false),
    ("snippets", false),
    ("reminder", false),
    ("paste_bins", false),
    ("whiteboards", false),
];

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = manager.get_database_backend();

        for (table, is_workspace) in SYNCABLE_TABLES {
            let (id_expr, now_expr) = match backend {
                DbBackend::Sqlite => ("hex(randomblob(13))", "datetime('now')"),
                _ => ("daily_ulid_fresh()", "NOW()"),
            };

            let ws_expr = if *is_workspace {
                format!("t.identifier")
            } else {
                format!("t.workspace_identifier")
            };

            let ws_guard = if *is_workspace {
                String::new()
            } else {
                " AND t.workspace_identifier IS NOT NULL".to_string()
            };

            let sql = format!(
                r#"INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
SELECT {id_expr}, '{table}', t.identifier,
       CASE WHEN t.deleted_at IS NOT NULL THEN 'DELETE' ELSE 'INSERT' END,
       {ws_expr}, {now_expr}, 0
FROM "{table}" t
WHERE NOT EXISTS (
    SELECT 1 FROM sync_queue sq
    WHERE sq.table_name = '{table}' AND sq.record_identifier = t.identifier
){ws_guard}"#
            );

            db.execute_unprepared(&sql).await?;
        }

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

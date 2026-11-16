use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database, DbBackend, Statement};

const SYNCABLE_TABLES: &[&str] = &[
    "notes",
    "task",
    "bookmark",
    "snippets",
    "reminder",
    "paste_bins",
    "whiteboards",
    "workspace_profiles",
];

#[tokio::test]
async fn sqlite_sync_foundation_migration_applies() {
    let path = std::env::temp_dir().join(format!(
        "almonds-migration-test-{}.db",
        uuid::Uuid::new_v4()
    ));
    let url = format!("sqlite://{}?mode=rwc", path.display());

    let db = Database::connect(&url).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    for table in SYNCABLE_TABLES {
        let rows = db
            .query_all_raw(Statement::from_string(
                DbBackend::Sqlite,
                format!("PRAGMA table_info(\"{table}\")"),
            ))
            .await
            .unwrap();
        let columns: Vec<String> = rows
            .iter()
            .map(|row| row.try_get::<String>("", "name").unwrap())
            .collect();
        assert!(
            columns.iter().any(|column| column == "deleted_at"),
            "{table} is missing deleted_at"
        );
    }

    let rows = db
        .query_all_raw(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT name FROM sqlite_master WHERE type = 'table'",
        ))
        .await
        .unwrap();
    let tables: Vec<String> = rows
        .iter()
        .map(|row| row.try_get::<String>("", "name").unwrap())
        .collect();
    for table in ["sync_queue", "sync_state", "sync_meta"] {
        assert!(
            tables.iter().any(|name| name == table),
            "{table} is missing"
        );
    }

    let rows = db
        .query_all_raw(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT name FROM sqlite_master WHERE type = 'trigger'",
        ))
        .await
        .unwrap();
    let triggers: Vec<String> = rows
        .iter()
        .map(|row| row.try_get::<String>("", "name").unwrap())
        .collect();
    for table in SYNCABLE_TABLES.iter().chain(["workspaces"].iter()) {
        for operation in ["insert", "update", "delete"] {
            let trigger = format!("{table}_sync_{operation}");
            assert!(
                triggers.iter().any(|name| name == &trigger),
                "{trigger} is missing"
            );
        }
    }

    let rows = db
        .query_all_raw(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT COUNT(*) AS count FROM seaql_migrations \
             WHERE version = 'm20261114_000000_sync_foundation'",
        ))
        .await
        .unwrap();
    assert_eq!(rows[0].try_get::<i64>("", "count").unwrap(), 1);

    db.close().await.unwrap();
    std::fs::remove_file(path).unwrap();
}

#[tokio::test]
async fn sqlite_ulid_migration_dedupes_and_converts_ids() {
    let path = std::env::temp_dir().join(format!(
        "almonds-ulid-migration-test-{}.db",
        uuid::Uuid::new_v4()
    ));
    let url = format!("sqlite://{}?mode=rwc", path.display());
    let db = Database::connect(&url).await.unwrap();

    let steps = (Migrator::migrations().len() - 1) as u32;
    Migrator::up(&db, Some(steps)).await.unwrap();

    db.execute_unprepared(
        r#"
        INSERT INTO workspaces(identifier, name, description, is_default, is_hidden, is_secured, created_at, updated_at)
        VALUES ('85b49460-f44a-47f4-930a-8d0b45ec34be', 'default', '', 1, 0, 0, '2026-01-01', '2026-01-01');
        INSERT INTO workspaces(identifier, name, description, is_default, is_hidden, is_secured, created_at, updated_at)
        VALUES ('85B49460-F44A-47F4-930A-8D0B45EC34BE', 'default', '', 1, 0, 0, '2026-01-01', '2026-01-01');
        INSERT INTO notes(identifier, title, content, created_at, updated_at, workspace_identifier)
        VALUES ('11111111-1111-4111-8111-111111111111', 'note', '', '2026-01-01', '2026-01-01', '85b49460-f44a-47f4-930a-8d0b45ec34be');
        INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
        VALUES ('22222222-2222-4222-8222-222222222222', 'notes', '11111111-1111-4111-8111-111111111111', 'INSERT', '85b49460-f44a-47f4-930a-8d0b45ec34be', '2026-01-01', 0);
        INSERT INTO sync_state(workspace_identifier, last_cursor) VALUES ('85b49460-f44a-47f4-930a-8d0b45ec34be', 1);
        "#,
    )
    .await
    .unwrap();

    Migrator::up(&db, None).await.unwrap();

    let workspaces = db
        .query_all_raw(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT identifier FROM workspaces",
        ))
        .await
        .unwrap();
    assert_eq!(workspaces.len(), 1, "duplicate workspace rows must be merged");
    let workspace: String = workspaces[0].try_get("", "identifier").unwrap();
    assert_eq!(workspace.len(), 26);

    let notes = db
        .query_all_raw(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT identifier, workspace_identifier FROM notes",
        ))
        .await
        .unwrap();
    let note: String = notes[0].try_get("", "identifier").unwrap();
    let note_workspace: String = notes[0].try_get("", "workspace_identifier").unwrap();
    assert_eq!(note.len(), 26);
    assert_eq!(note_workspace, workspace);

    let queue = db
        .query_all_raw(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT identifier, record_identifier, workspace_identifier FROM sync_queue WHERE table_name = 'notes' AND operation = 'INSERT'",
        ))
        .await
        .unwrap();
    let queue_id: String = queue[0].try_get("", "identifier").unwrap();
    assert_eq!(queue_id.len(), 26);
    assert_eq!(queue_id, queue_id.to_ascii_uppercase());
    assert_eq!(queue[0].try_get::<String>("", "record_identifier").unwrap(), note);
    assert_eq!(queue[0].try_get::<String>("", "workspace_identifier").unwrap(), workspace);

    db.execute_unprepared(
        "INSERT INTO task(identifier, title, created_at, updated_at, workspace_identifier) \
         VALUES ('01JZZZZZZZZZZZZZZZZZZZZZZZ', 'task', '2026-01-01', '2026-01-01', NULL)",
    )
    .await
    .unwrap();

    let queue = db
        .query_all_raw(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT record_identifier FROM sync_queue WHERE table_name = 'task' ORDER BY rowid DESC LIMIT 1",
        ))
        .await
        .unwrap();
    let record_identifier: String = queue[0].try_get("", "record_identifier").unwrap();
    assert_eq!(record_identifier.len(), 26);

    db.close().await.unwrap();
    std::fs::remove_file(path).unwrap();
}

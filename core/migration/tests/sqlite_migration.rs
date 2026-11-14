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

use std::collections::HashMap;

use sea_orm::{ConnectionTrait, DbBackend, Statement};
use sea_orm_migration::prelude::*;
use ulid::Ulid;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

const ID_COLUMNS: &[(&str, &str)] = &[
    ("notes", "identifier"),
    ("notes", "workspace_identifier"),
    ("task", "identifier"),
    ("task", "workspace_identifier"),
    ("bookmark", "identifier"),
    ("bookmark", "workspace_identifier"),
    ("snippets", "identifier"),
    ("snippets", "workspace_identifier"),
    ("reminder", "identifier"),
    ("reminder", "workspace_identifier"),
    ("paste_bins", "identifier"),
    ("paste_bins", "workspace_identifier"),
    ("whiteboards", "identifier"),
    ("whiteboards", "workspace_identifier"),
    ("workspace_profiles", "identifier"),
    ("workspace_profiles", "workspace_identifier"),
    ("workspace_members", "identifier"),
    ("workspace_members", "workspace_identifier"),
    ("workspace_members", "user_identifier"),
    ("notifications", "identifier"),
    ("notifications", "workspace_identifier"),
    ("recycle_bin", "identifier"),
    ("recycle_bin", "item_id"),
    ("recycle_bin", "workspace_identifier"),
    ("note_categories", "identifier"),
    ("user_preferences", "identifier"),
    ("users", "identifier"),
    ("one_time_password", "identifier"),
    ("one_time_password", "user_identifier"),
    ("revoked_token", "identifier"),
    ("revoked_token", "jti"),
    ("revoked_token", "user_identifier"),
    ("invitation", "identifier"),
    ("invitation", "workspace_identifier"),
    ("workspaces", "identifier"),
    ("workspaces", "user_identifier"),
    ("whiteboard_files", "identifier"),
    ("whiteboard_files", "whiteboard_identifier"),
    ("sync_queue", "identifier"),
    ("sync_queue", "record_identifier"),
    ("sync_queue", "workspace_identifier"),
    ("sync_state", "workspace_identifier"),
];

const PRIMARY_TABLES: &[&str] = &[
    "notes",
    "task",
    "bookmark",
    "snippets",
    "reminder",
    "paste_bins",
    "whiteboards",
    "workspace_profiles",
    "workspace_members",
    "notifications",
    "recycle_bin",
    "note_categories",
    "user_preferences",
    "users",
    "one_time_password",
    "revoked_token",
    "invitation",
    "workspaces",
];

const RELATIONS: &[(&str, &str, &str, &str)] = &[
    ("workspaces", "identifier", "notes", "workspace_identifier"),
    ("workspaces", "identifier", "task", "workspace_identifier"),
    (
        "workspaces",
        "identifier",
        "bookmark",
        "workspace_identifier",
    ),
    (
        "workspaces",
        "identifier",
        "snippets",
        "workspace_identifier",
    ),
    (
        "workspaces",
        "identifier",
        "reminder",
        "workspace_identifier",
    ),
    (
        "workspaces",
        "identifier",
        "paste_bins",
        "workspace_identifier",
    ),
    (
        "workspaces",
        "identifier",
        "whiteboards",
        "workspace_identifier",
    ),
    (
        "workspaces",
        "identifier",
        "workspace_profiles",
        "workspace_identifier",
    ),
    (
        "workspaces",
        "identifier",
        "workspace_members",
        "workspace_identifier",
    ),
    (
        "workspaces",
        "identifier",
        "notifications",
        "workspace_identifier",
    ),
    (
        "workspaces",
        "identifier",
        "recycle_bin",
        "workspace_identifier",
    ),
    (
        "workspaces",
        "identifier",
        "invitation",
        "workspace_identifier",
    ),
    ("users", "identifier", "workspaces", "user_identifier"),
    (
        "users",
        "identifier",
        "workspace_members",
        "user_identifier",
    ),
    (
        "users",
        "identifier",
        "one_time_password",
        "user_identifier",
    ),
    ("users", "identifier", "revoked_token", "user_identifier"),
    (
        "whiteboards",
        "identifier",
        "whiteboard_files",
        "whiteboard_identifier",
    ),
];

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
    fn use_transaction(&self) -> Option<bool> {
        Some(true)
    }

    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        match manager.get_database_backend() {
            DbBackend::Sqlite => self.up_sqlite(manager).await,
            DbBackend::Postgres => manager
                .get_connection()
                .execute_unprepared(POSTGRES_UP)
                .await
                .map(|_| ()),
            _ => Ok(()),
        }
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

impl Migration {
    async fn up_sqlite(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("PRAGMA defer_foreign_keys = ON;")
            .await?;
        self.drop_sqlite_triggers(db).await?;
        self.dedupe_sqlite(db).await?;

        db.execute_unprepared(&format!(
            r#"DELETE FROM sync_state
               WHERE rowid NOT IN (
                   SELECT rowid FROM (
                       SELECT rowid,
                              ROW_NUMBER() OVER (
                                  PARTITION BY {}
                                  ORDER BY last_cursor DESC, COALESCE(last_synced_at, '') DESC, rowid
                              ) AS rn
                       FROM sync_state
                   ) WHERE rn = 1
               );"#,
            sqlite_key_expr("workspace_identifier")
        ))
        .await?;

        let mut ids = HashMap::new();
        for table in PRIMARY_TABLES {
            for (_, kind, raw) in self.sqlite_rows(db, table, "identifier").await?.into_iter() {
                let key = canonical_id(&kind, &raw);
                ids.entry(key).or_insert_with_key(|key| to_ulid(key));
            }
        }

        for (table, column) in ID_COLUMNS {
            for (rowid, kind, raw) in self.sqlite_rows(db, table, column).await?.into_iter() {
                let key = canonical_id(&kind, &raw);
                let new_id = if *table == "sync_queue" && *column == "identifier" {
                    new_ulid()
                } else if *table == "whiteboard_files" && *column == "identifier" {
                    opaque_id(&kind, &raw)
                } else {
                    ids.get(&key).cloned().unwrap_or_else(|| to_ulid(&key))
                };
                db.execute_raw(Statement::from_sql_and_values(
                    DbBackend::Sqlite,
                    format!("UPDATE \"{table}\" SET \"{column}\" = ? WHERE rowid = ?"),
                    [new_id.into(), rowid.into()],
                ))
                .await?;
            }
        }

        self.rewrite_sqlite_payloads(db).await?;
        self.create_sqlite_triggers(db).await?;
        Ok(())
    }

    async fn sqlite_rows(
        &self,
        db: &sea_orm_migration::SchemaManagerConnection<'_>,
        table: &str,
        column: &str,
    ) -> Result<Vec<(i64, String, String)>, DbErr> {
        let rows = db
            .query_all_raw(Statement::from_string(
                DbBackend::Sqlite,
                format!(
                    r#"SELECT rowid AS rid, typeof("{column}") AS kind,
                              CASE WHEN typeof("{column}") = 'blob'
                                   THEN hex("{column}") ELSE CAST("{column}" AS TEXT) END AS raw
                       FROM "{table}""#
                ),
            ))
            .await?;

        Ok(rows
            .into_iter()
            .filter_map(|row| {
                let kind = row.try_get::<String>("", "kind").ok()?;
                let raw = row.try_get::<String>("", "raw").ok()?;
                let rowid = row.try_get::<i64>("", "rid").ok()?;
                Some((rowid, kind, raw))
            })
            .collect())
    }

    async fn dedupe_sqlite(
        &self,
        db: &sea_orm_migration::SchemaManagerConnection<'_>,
    ) -> Result<(), DbErr> {
        for table in PRIMARY_TABLES {
            let mut groups: HashMap<String, Vec<(i64, String)>> = HashMap::new();
            for (rowid, kind, raw) in self.sqlite_rows(db, table, "identifier").await?.into_iter() {
                groups
                    .entry(canonical_id(&kind, &raw))
                    .or_default()
                    .push((rowid, kind));
            }

            for (key, mut rows) in groups {
                if rows.len() < 2 {
                    continue;
                }
                rows.sort_by_key(|(_, kind)| if kind == "text" { 0 } else { 1 });
                let winner = rows[0].0;
                for (loser, _) in rows.iter().skip(1) {
                    for (parent, _, child, child_column) in RELATIONS {
                        if *parent != *table {
                            continue;
                        }
                        db.execute_raw(Statement::from_sql_and_values(
                            DbBackend::Sqlite,
                            format!(
                                r#"UPDATE "{child}" SET "{child_column}" =
                                       (SELECT "identifier" FROM "{table}" WHERE rowid = ?)
                                   WHERE {} = ?"#,
                                sqlite_key_expr(child_column)
                            ),
                            [winner.into(), key.clone().into()],
                        ))
                        .await?;
                    }
                    db.execute_raw(Statement::from_sql_and_values(
                        DbBackend::Sqlite,
                        format!("DELETE FROM \"{table}\" WHERE rowid = ?"),
                        [(*loser).into()],
                    ))
                    .await?;
                }
            }
        }
        Ok(())
    }

    async fn rewrite_sqlite_payloads(
        &self,
        db: &sea_orm_migration::SchemaManagerConnection<'_>,
    ) -> Result<(), DbErr> {
        let rows = db
            .query_all_raw(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT rowid AS rid, payload FROM recycle_bin WHERE payload IS NOT NULL",
            ))
            .await?;
        for row in rows {
            let Some(rowid) = row.try_get::<i64>("", "rid").ok() else {
                continue;
            };
            let Some(payload) = row.try_get::<String>("", "payload").ok() else {
                continue;
            };
            let Ok(mut value) = serde_json::from_str::<serde_json::Value>(&payload) else {
                continue;
            };
            if rewrite_json_ids(&mut value) {
                db.execute_raw(Statement::from_sql_and_values(
                    DbBackend::Sqlite,
                    "UPDATE recycle_bin SET payload = ? WHERE rowid = ?",
                    [value.to_string().into(), rowid.into()],
                ))
                .await?;
            }
        }
        Ok(())
    }

    async fn drop_sqlite_triggers(
        &self,
        db: &sea_orm_migration::SchemaManagerConnection<'_>,
    ) -> Result<(), DbErr> {
        for (table, _) in SYNCABLE_TABLES {
            db.execute_unprepared(&format!(
                "DROP TRIGGER IF EXISTS {table}_sync_insert;
                 DROP TRIGGER IF EXISTS {table}_sync_update;
                 DROP TRIGGER IF EXISTS {table}_sync_delete;"
            ))
            .await?;
        }
        Ok(())
    }

    async fn create_sqlite_triggers(
        &self,
        db: &sea_orm_migration::SchemaManagerConnection<'_>,
    ) -> Result<(), DbErr> {
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
            db.execute_unprepared(&format!(
                r#"
DROP TRIGGER IF EXISTS {table}_sync_insert;
CREATE TRIGGER {table}_sync_insert
AFTER INSERT ON "{table}"
FOR EACH ROW
BEGIN
    INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
    SELECT upper(hex(randomblob(13))), '{table}', NEW.identifier, 'INSERT', {insert_ws}, datetime('now'), 0
    WHERE (SELECT value FROM sync_meta WHERE key = 'applying') IS NULL;
END;

DROP TRIGGER IF EXISTS {table}_sync_update;
CREATE TRIGGER {table}_sync_update
AFTER UPDATE ON "{table}"
FOR EACH ROW
BEGIN
    INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
    SELECT upper(hex(randomblob(13))), '{table}', NEW.identifier, 'UPDATE', {update_ws}, datetime('now'), 0
    WHERE (SELECT value FROM sync_meta WHERE key = 'applying') IS NULL;
END;

DROP TRIGGER IF EXISTS {table}_sync_delete;
CREATE TRIGGER {table}_sync_delete
AFTER DELETE ON "{table}"
FOR EACH ROW
BEGIN
    INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
    SELECT upper(hex(randomblob(13))), '{table}', OLD.identifier, 'DELETE', {delete_ws}, datetime('now'), 0
    WHERE (SELECT value FROM sync_meta WHERE key = 'applying') IS NULL;
END;
"#
            ))
            .await?;
        }
        Ok(())
    }
}

fn sqlite_key_expr(column: &str) -> String {
    format!(
        r#"CASE
             WHEN typeof("{column}") = 'blob' THEN lower(hex("{column}"))
             WHEN length(replace(lower(CAST("{column}" AS TEXT)), '-', '')) = 32
                 THEN lower(replace(CAST("{column}" AS TEXT), '-', ''))
             WHEN length(CAST("{column}" AS TEXT)) = 26
                 THEN upper(CAST("{column}" AS TEXT))
             ELSE lower(CAST("{column}" AS TEXT))
           END"#
    )
}

fn canonical_id(kind: &str, raw: &str) -> String {
    let raw = raw.trim();
    if kind == "blob" {
        if let Ok(bytes) = hex::decode(raw) {
            if bytes.len() == 16 {
                return hex::encode(bytes);
            }
        }
        return raw.to_ascii_lowercase();
    }

    if let Ok(uuid) = Uuid::parse_str(raw) {
        return uuid.simple().to_string();
    }
    if raw.len() == 32 && raw.chars().all(|c| c.is_ascii_hexdigit()) {
        return raw.to_ascii_lowercase();
    }
    if raw.len() == 72 && raw.chars().all(|c| c.is_ascii_hexdigit()) {
        if let Ok(bytes) = hex::decode(raw) {
            if let Ok(value) = String::from_utf8(bytes) {
                if let Ok(uuid) = Uuid::parse_str(value.trim()) {
                    return uuid.simple().to_string();
                }
            }
        }
    }
    if raw.len() == 26
        && raw
            .chars()
            .all(|c| "0123456789ABCDEFGHJKMNPQRSTVWXYZabcdefghjkmnpqrstvwxyz".contains(c))
    {
        return raw.to_ascii_uppercase();
    }
    raw.to_string()
}

fn to_ulid(key: &str) -> String {
    if key.len() == 26
        && key
            .chars()
            .all(|c| "0123456789ABCDEFGHJKMNPQRSTVWXYZ".contains(c))
    {
        return key.to_string();
    }
    if key.len() == 32 && key.chars().all(|c| c.is_ascii_hexdigit()) {
        if let Ok(bytes) = hex::decode(key) {
            if bytes.len() == 16 {
                return Ulid::from_bytes(bytes.try_into().unwrap()).to_string();
            }
        }
    }
    Uuid::parse_str(key)
        .map(|uuid| Ulid::from_bytes(*uuid.as_bytes()).to_string())
        .unwrap_or_else(|_| key.to_string())
}

fn opaque_id(kind: &str, raw: &str) -> String {
    if kind == "blob" {
        if let Ok(bytes) = hex::decode(raw.trim()) {
            if bytes.len() == 16 {
                return Uuid::from_slice(&bytes).unwrap().to_string();
            }
        }
    }
    raw.trim().to_string()
}

fn new_ulid() -> String {
    Ulid::generate().to_string()
}

fn rewrite_json_ids(value: &mut serde_json::Value) -> bool {
    const ID_KEYS: &[&str] = &[
        "identifier",
        "workspace_identifier",
        "record_identifier",
        "item_id",
        "item_identifier",
        "user_identifier",
        "jti",
        "previous_workspace_identifier",
        "target_workspace_identifier",
    ];

    match value {
        serde_json::Value::Object(map) => {
            let mut changed = false;
            for (key, child) in map.iter_mut() {
                if ID_KEYS.contains(&key.as_str()) {
                    if let serde_json::Value::String(current) = child {
                        let next = to_ulid(&canonical_id("text", current));
                        if *current != next {
                            *current = next;
                            changed = true;
                        }
                        continue;
                    }
                }
                changed |= rewrite_json_ids(child);
            }
            changed
        }
        serde_json::Value::Array(items) => {
            let mut changed = false;
            for item in items {
                changed |= rewrite_json_ids(item);
            }
            changed
        }
        _ => false,
    }
}

const POSTGRES_UP: &str = r##"
CREATE OR REPLACE FUNCTION daily_uuid_to_ulid(input uuid)
RETURNS text LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE
    alphabet CONSTANT text := '0123456789ABCDEFGHJKMNPQRSTVWXYZ';
    padded bit(130);
    idx integer;
    result text := '';
    i integer;
    j integer;
BEGIN
    IF input IS NULL THEN
        RETURN NULL;
    END IF;
    padded := B'00' || ('x' || replace(input::text, '-', ''))::bit(128);
    FOR i IN 0..25 LOOP
        idx := 0;
        FOR j IN 0..4 LOOP
            idx := idx * 2 + get_bit(padded, i * 5 + j);
        END LOOP;
        result := result || substr(alphabet, idx + 1, 1);
    END LOOP;
    RETURN result;
END;
$$;

CREATE OR REPLACE FUNCTION daily_text_to_ulid(input text)
RETURNS text LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE
    value text;
    compact text;
BEGIN
    IF input IS NULL THEN
        RETURN NULL;
    END IF;
    value := btrim(input);
    IF value = '' THEN
        RETURN value;
    END IF;
    IF length(value) = 26 AND value ~* '^[0-9A-HJKMNP-TV-Z]{26}$' THEN
        RETURN upper(value);
    END IF;
    compact := replace(lower(value), '-', '');
    IF length(compact) = 32 AND compact ~ '^[0-9a-f]{32}$' THEN
        RETURN daily_uuid_to_ulid((
            substr(compact, 1, 8) || '-' || substr(compact, 9, 4) || '-' ||
            substr(compact, 13, 4) || '-' || substr(compact, 17, 4) || '-' ||
            substr(compact, 21, 12)
        )::uuid);
    END IF;
    RETURN value;
END;
$$;

CREATE OR REPLACE FUNCTION daily_ulid_fresh()
RETURNS text LANGUAGE sql VOLATILE AS $$
    SELECT daily_uuid_to_ulid(gen_random_uuid());
$$;

CREATE OR REPLACE FUNCTION daily_ulidify_json(input jsonb)
RETURNS jsonb LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE
    result jsonb;
BEGIN
    IF input IS NULL THEN
        RETURN NULL;
    END IF;
    IF jsonb_typeof(input) = 'object' THEN
        SELECT jsonb_object_agg(
            key,
            CASE
                WHEN key IN ('identifier', 'workspace_identifier', 'record_identifier',
                             'item_id', 'item_identifier', 'user_identifier', 'jti',
                             'previous_workspace_identifier', 'target_workspace_identifier')
                     AND jsonb_typeof(value) = 'string'
                    THEN to_jsonb(daily_text_to_ulid(value #>> '{}'))
                WHEN jsonb_typeof(value) IN ('object', 'array')
                    THEN daily_ulidify_json(value)
                ELSE value
            END
        ) INTO result
        FROM jsonb_each(input);
        RETURN COALESCE(result, '{}'::jsonb);
    ELSIF jsonb_typeof(input) = 'array' THEN
        SELECT jsonb_agg(
            CASE
                WHEN jsonb_typeof(value) IN ('object', 'array')
                    THEN daily_ulidify_json(value)
                ELSE value
            END
        ) INTO result
        FROM jsonb_array_elements(input);
        RETURN COALESCE(result, '[]'::jsonb);
    END IF;
    RETURN input;
END;
$$;

CREATE TEMP TABLE _daily_ulid_columns (
    table_name text NOT NULL,
    column_name text NOT NULL,
    mode text NOT NULL
) ON COMMIT DROP;

INSERT INTO _daily_ulid_columns(table_name, column_name, mode) VALUES
    ('notes', 'identifier', 'ulid'),
    ('notes', 'workspace_identifier', 'ulid'),
    ('task', 'identifier', 'ulid'),
    ('task', 'workspace_identifier', 'ulid'),
    ('bookmark', 'identifier', 'ulid'),
    ('bookmark', 'workspace_identifier', 'ulid'),
    ('snippets', 'identifier', 'ulid'),
    ('snippets', 'workspace_identifier', 'ulid'),
    ('reminder', 'identifier', 'ulid'),
    ('reminder', 'workspace_identifier', 'ulid'),
    ('paste_bins', 'identifier', 'ulid'),
    ('paste_bins', 'workspace_identifier', 'ulid'),
    ('whiteboards', 'identifier', 'ulid'),
    ('whiteboards', 'workspace_identifier', 'ulid'),
    ('workspace_profiles', 'identifier', 'ulid'),
    ('workspace_profiles', 'workspace_identifier', 'ulid'),
    ('workspace_members', 'identifier', 'ulid'),
    ('workspace_members', 'workspace_identifier', 'ulid'),
    ('workspace_members', 'user_identifier', 'ulid'),
    ('notifications', 'identifier', 'ulid'),
    ('notifications', 'workspace_identifier', 'ulid'),
    ('recycle_bin', 'identifier', 'ulid'),
    ('recycle_bin', 'item_id', 'ulid'),
    ('recycle_bin', 'workspace_identifier', 'ulid'),
    ('note_categories', 'identifier', 'ulid'),
    ('user_preferences', 'identifier', 'ulid'),
    ('users', 'identifier', 'ulid'),
    ('one_time_password', 'identifier', 'ulid'),
    ('one_time_password', 'user_identifier', 'ulid'),
    ('revoked_token', 'identifier', 'ulid'),
    ('revoked_token', 'jti', 'ulid'),
    ('revoked_token', 'user_identifier', 'ulid'),
    ('invitation', 'identifier', 'ulid'),
    ('invitation', 'workspace_identifier', 'ulid'),
    ('workspaces', 'identifier', 'ulid'),
    ('workspaces', 'user_identifier', 'ulid'),
    ('whiteboard_files', 'identifier', 'opaque'),
    ('whiteboard_files', 'whiteboard_identifier', 'ulid'),
    ('sync_queue', 'identifier', 'fresh'),
    ('sync_queue', 'record_identifier', 'ulid'),
    ('sync_queue', 'workspace_identifier', 'ulid'),
    ('sync_state', 'workspace_identifier', 'ulid');

CREATE TEMP TABLE _daily_ulid_parents (table_name text PRIMARY KEY) ON COMMIT DROP;
INSERT INTO _daily_ulid_parents(table_name) VALUES
    ('notes'), ('task'), ('bookmark'), ('snippets'), ('reminder'), ('paste_bins'),
    ('whiteboards'), ('workspace_profiles'), ('workspace_members'), ('notifications'),
    ('recycle_bin'), ('note_categories'), ('user_preferences'), ('users'),
    ('one_time_password'), ('revoked_token'), ('invitation'), ('workspaces');

CREATE TEMP TABLE _daily_ulid_relations (
    parent_table text NOT NULL,
    parent_column text NOT NULL,
    child_table text NOT NULL,
    child_column text NOT NULL
) ON COMMIT DROP;
INSERT INTO _daily_ulid_relations VALUES
    ('workspaces', 'identifier', 'notes', 'workspace_identifier'),
    ('workspaces', 'identifier', 'task', 'workspace_identifier'),
    ('workspaces', 'identifier', 'bookmark', 'workspace_identifier'),
    ('workspaces', 'identifier', 'snippets', 'workspace_identifier'),
    ('workspaces', 'identifier', 'reminder', 'workspace_identifier'),
    ('workspaces', 'identifier', 'paste_bins', 'workspace_identifier'),
    ('workspaces', 'identifier', 'whiteboards', 'workspace_identifier'),
    ('workspaces', 'identifier', 'workspace_profiles', 'workspace_identifier'),
    ('workspaces', 'identifier', 'workspace_members', 'workspace_identifier'),
    ('workspaces', 'identifier', 'notifications', 'workspace_identifier'),
    ('workspaces', 'identifier', 'recycle_bin', 'workspace_identifier'),
    ('workspaces', 'identifier', 'invitation', 'workspace_identifier'),
    ('users', 'identifier', 'workspaces', 'user_identifier'),
    ('users', 'identifier', 'workspace_members', 'user_identifier'),
    ('users', 'identifier', 'one_time_password', 'user_identifier'),
    ('users', 'identifier', 'revoked_token', 'user_identifier'),
    ('whiteboards', 'identifier', 'whiteboard_files', 'whiteboard_identifier');

CREATE TEMP TABLE _daily_ulid_fks ON COMMIT DROP AS
SELECT conrelid::regclass::text AS table_name,
       conname,
       pg_get_constraintdef(oid) AS definition
FROM pg_constraint
WHERE contype = 'f' AND connamespace = 'public'::regnamespace;

DO $$
DECLARE
    fk record;
BEGIN
    FOR fk IN SELECT * FROM _daily_ulid_fks LOOP
        EXECUTE format('ALTER TABLE %s DROP CONSTRAINT %I', fk.table_name, fk.conname);
    END LOOP;
END;
$$;

DO $$
DECLARE
    item record;
BEGIN
    FOR item IN SELECT * FROM _daily_ulid_columns ORDER BY table_name, column_name LOOP
        EXECUTE format(
            'ALTER TABLE %I ALTER COLUMN %I TYPE text USING %I::text',
            item.table_name, item.column_name, item.column_name
        );
    END LOOP;
END;
$$;

DO $$
DECLARE
    parent record;
    dup record;
    relation record;
    winner tid;
    loser tid;
    i integer;
BEGIN
    FOR parent IN SELECT table_name FROM _daily_ulid_parents LOOP
        FOR dup IN EXECUTE format(
            'SELECT identifier, array_agg(ctid ORDER BY ctid) AS rows
             FROM %I GROUP BY identifier HAVING count(*) > 1',
            parent.table_name
        ) LOOP
            winner := dup.rows[1];
            FOR i IN 2..array_length(dup.rows, 1) LOOP
                loser := dup.rows[i];
                FOR relation IN
                    SELECT * FROM _daily_ulid_relations WHERE parent_table = parent.table_name
                LOOP
                    EXECUTE format(
                        'UPDATE %I SET %I = (SELECT %I FROM %I WHERE ctid = $1)
                         WHERE %I = (SELECT %I FROM %I WHERE ctid = $2)',
                        relation.child_table, relation.child_column,
                        relation.parent_column, parent.table_name,
                        relation.child_column, relation.parent_column, parent.table_name
                    ) USING winner, loser;
                END LOOP;
                EXECUTE format('DELETE FROM %I WHERE ctid = $1', parent.table_name)
                USING loser;
            END LOOP;
        END LOOP;
    END LOOP;

    DELETE FROM sync_state a USING sync_state b
    WHERE a.workspace_identifier = b.workspace_identifier
      AND (a.last_cursor < b.last_cursor
           OR (a.last_cursor = b.last_cursor AND a.ctid > b.ctid));
END;
$$;

DO $$
DECLARE
    item record;
BEGIN
    FOR item IN SELECT * FROM _daily_ulid_columns ORDER BY table_name, column_name LOOP
        IF item.mode = 'fresh' THEN
            EXECUTE format(
                'ALTER TABLE %I ALTER COLUMN %I TYPE varchar(26) USING daily_ulid_fresh()',
                item.table_name, item.column_name
            );
        ELSIF item.mode = 'opaque' THEN
            EXECUTE format(
                'ALTER TABLE %I ALTER COLUMN %I TYPE varchar(64) USING %I',
                item.table_name, item.column_name, item.column_name
            );
        ELSE
            EXECUTE format(
                'ALTER TABLE %I ALTER COLUMN %I TYPE varchar(26) USING daily_text_to_ulid(%I)',
                item.table_name, item.column_name, item.column_name
            );
        END IF;
    END LOOP;
END;
$$;

UPDATE recycle_bin
SET payload = daily_ulidify_json(payload::jsonb)::text
WHERE payload IS NOT NULL AND payload ~ '^\s*[\{\[]';

DO $$
DECLARE
    fk record;
BEGIN
    FOR fk IN SELECT * FROM _daily_ulid_fks LOOP
        EXECUTE format('ALTER TABLE %s ADD CONSTRAINT %I %s', fk.table_name, fk.conname, fk.definition);
    END LOOP;
END;
$$;

DROP TRIGGER IF EXISTS notes_sync_trigger ON notes;
DROP TRIGGER IF EXISTS task_sync_trigger ON task;
DROP TRIGGER IF EXISTS bookmark_sync_trigger ON bookmark;
DROP TRIGGER IF EXISTS snippets_sync_trigger ON snippets;
DROP TRIGGER IF EXISTS reminder_sync_trigger ON reminder;
DROP TRIGGER IF EXISTS paste_bins_sync_trigger ON paste_bins;
DROP TRIGGER IF EXISTS whiteboards_sync_trigger ON whiteboards;
DROP TRIGGER IF EXISTS workspace_profiles_sync_trigger ON workspace_profiles;
DROP TRIGGER IF EXISTS workspaces_sync_trigger ON workspaces;
DROP TRIGGER IF EXISTS notes_sync_log_trigger ON notes;
DROP TRIGGER IF EXISTS task_sync_log_trigger ON task;
DROP TRIGGER IF EXISTS bookmark_sync_log_trigger ON bookmark;
DROP TRIGGER IF EXISTS snippets_sync_log_trigger ON snippets;
DROP TRIGGER IF EXISTS reminder_sync_log_trigger ON reminder;
DROP TRIGGER IF EXISTS paste_bins_sync_log_trigger ON paste_bins;
DROP TRIGGER IF EXISTS whiteboards_sync_log_trigger ON whiteboards;
DROP TRIGGER IF EXISTS workspace_profiles_sync_log_trigger ON workspace_profiles;
DROP TRIGGER IF EXISTS workspaces_sync_log_trigger ON workspaces;
DROP FUNCTION IF EXISTS enqueue_sync();
DROP FUNCTION IF EXISTS log_sync_change();

CREATE OR REPLACE FUNCTION enqueue_sync()
RETURNS TRIGGER AS $$
DECLARE
    applying text;
    rec_id text;
    ws_id text;
BEGIN
    SELECT value INTO applying FROM sync_meta WHERE key = 'applying';
    IF applying IS NOT NULL AND applying <> '' THEN
        RETURN COALESCE(NEW, OLD);
    END IF;

    IF TG_OP = 'DELETE' THEN
        rec_id := OLD.identifier::text;
        ws_id := COALESCE(to_jsonb(OLD) ->> 'workspace_identifier', OLD.identifier::text);
    ELSE
        rec_id := NEW.identifier::text;
        ws_id := COALESCE(to_jsonb(NEW) ->> 'workspace_identifier', NEW.identifier::text);
    END IF;

    INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
    VALUES (daily_ulid_fresh(), TG_TABLE_NAME, rec_id, TG_OP, ws_id, NOW(), 0);
    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER notes_sync_trigger AFTER INSERT OR UPDATE OR DELETE ON notes FOR EACH ROW EXECUTE FUNCTION enqueue_sync();
CREATE TRIGGER task_sync_trigger AFTER INSERT OR UPDATE OR DELETE ON task FOR EACH ROW EXECUTE FUNCTION enqueue_sync();
CREATE TRIGGER bookmark_sync_trigger AFTER INSERT OR UPDATE OR DELETE ON bookmark FOR EACH ROW EXECUTE FUNCTION enqueue_sync();
CREATE TRIGGER snippets_sync_trigger AFTER INSERT OR UPDATE OR DELETE ON snippets FOR EACH ROW EXECUTE FUNCTION enqueue_sync();
CREATE TRIGGER reminder_sync_trigger AFTER INSERT OR UPDATE OR DELETE ON reminder FOR EACH ROW EXECUTE FUNCTION enqueue_sync();
CREATE TRIGGER paste_bins_sync_trigger AFTER INSERT OR UPDATE OR DELETE ON paste_bins FOR EACH ROW EXECUTE FUNCTION enqueue_sync();
CREATE TRIGGER whiteboards_sync_trigger AFTER INSERT OR UPDATE OR DELETE ON whiteboards FOR EACH ROW EXECUTE FUNCTION enqueue_sync();
CREATE TRIGGER workspace_profiles_sync_trigger AFTER INSERT OR UPDATE OR DELETE ON workspace_profiles FOR EACH ROW EXECUTE FUNCTION enqueue_sync();
CREATE TRIGGER workspaces_sync_trigger AFTER INSERT OR UPDATE OR DELETE ON workspaces FOR EACH ROW EXECUTE FUNCTION enqueue_sync();
"##;

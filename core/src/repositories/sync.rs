use std::collections::HashSet;
use std::sync::Arc;

use chrono::Utc;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement, sea_query::Value};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::KernelError;

const DEVICE_KEY: &str = "device_id";
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
const RECYCLE_BIN_TABLES: &[(&str, &str)] = &[
    ("notes", "note"),
    ("task", "todo"),
    ("bookmark", "bookmark"),
    ("snippets", "snippet"),
    ("reminder", "reminder"),
    ("paste_bins", "paste_bin"),
];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncQueueItem {
    pub identifier: String,
    pub table_name: String,
    pub record_identifier: String,
    pub operation: String,
    pub workspace_identifier: Option<Uuid>,
    pub created_at: String,
    pub attempts: i32,
    pub last_error: Option<String>,
    pub device_id: Option<String>,
    pub payload: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncState {
    pub workspace_identifier: Uuid,
    pub last_cursor: String,
    pub last_synced_at: Option<String>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteSyncChange {
    pub table_name: String,
    pub record_identifier: Uuid,
    pub operation: String,
    #[serde(default)]
    pub payload: Option<serde_json::Value>,
    #[serde(default)]
    pub seq: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

impl RemoteSyncChange {
    /// Timestamp used to order a delete against local edits. Pull payloads
    /// carry the row that was deleted, so prefer its `deleted_at`, then its
    /// `updated_at`; fall back to the pull-level timestamp.
    fn tombstone_timestamp(&self) -> String {
        self.payload
            .as_ref()
            .and_then(|payload| {
                payload
                    .get("deleted_at")
                    .or_else(|| payload.get("updated_at"))
                    .and_then(|value| value.as_str())
            })
            .map(str::to_string)
            .or_else(|| self.updated_at.clone())
            .unwrap_or_else(|| Utc::now().to_rfc3339())
    }
}

fn tombstone_payload(
    change: &RemoteSyncChange,
    timestamp: &str,
    workspace_identifier: Uuid,
    is_workspace: bool,
) -> serde_json::Value {
    let mut object = change
        .payload
        .clone()
        .and_then(|value| value.as_object().cloned())
        .unwrap_or_default();
    object.insert(
        "identifier".to_string(),
        serde_json::Value::String(change.record_identifier.to_string()),
    );
    if !is_workspace {
        object.insert(
            "workspace_identifier".to_string(),
            serde_json::Value::String(workspace_identifier.to_string()),
        );
    }
    object.insert(
        "deleted_at".to_string(),
        serde_json::Value::String(timestamp.to_string()),
    );
    if !object.contains_key("updated_at") {
        object.insert(
            "updated_at".to_string(),
            serde_json::Value::String(timestamp.to_string()),
        );
    }
    serde_json::Value::Object(object)
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyChangesResult {
    pub applied: usize,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SyncRepository {
    conn: Arc<DatabaseConnection>,
}

impl SyncRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    fn backend(&self) -> DbBackend {
        self.conn.get_database_backend()
    }

    fn placeholder(&self, index: usize) -> String {
        match self.backend() {
            DbBackend::Postgres => format!("${index}"),
            _ => "?".to_string(),
        }
    }

    fn db_error(err: impl std::fmt::Display) -> KernelError {
        KernelError::DbOperationError(err.to_string())
    }

    pub async fn queue(
        &self,
        workspace_identifier: Uuid,
    ) -> Result<Vec<SyncQueueItem>, KernelError> {
        let sql = r#"
            SELECT
                lower(hex(identifier)) AS identifier,
                table_name,
                lower(record_identifier) AS record_identifier,
                operation,
                workspace_identifier,
                created_at,
                attempts,
                last_error,
                device_id
            FROM sync_queue
            WHERE workspace_identifier = ?
            ORDER BY created_at ASC, identifier ASC
        "#;
        let stmt = Statement::from_sql_and_values(
            self.backend(),
            sql,
            [Value::Uuid(Some(workspace_identifier))],
        );
        let rows = self
            .conn
            .query_all_raw(stmt)
            .await
            .map_err(Self::db_error)?;

        //TODO: consider using rayon and 
        let mut items = Vec::with_capacity(rows.len());
        for row in rows {
            let table_name: String = row.try_get("", "table_name").map_err(Self::db_error)?;
            let record_identifier: String = row
                .try_get("", "record_identifier")
                .map_err(Self::db_error)?;
            let operation: String = row.try_get("", "operation").map_err(Self::db_error)?;
            let created_at: String = row.try_get("", "created_at").map_err(Self::db_error)?;
            let payload = match self.row_payload(&table_name, &record_identifier).await? {
                Some(payload) => Some(payload),
                None if operation == "DELETE" => {
                    let timestamp = created_at.clone();
                    let mut object = serde_json::Map::new();
                    object.insert(
                        "identifier".to_string(),
                        serde_json::Value::String(record_identifier.clone()),
                    );
                    if table_name != "workspaces" {
                        object.insert(
                            "workspace_identifier".to_string(),
                            serde_json::Value::String(workspace_identifier.to_string()),
                        );
                    }
                    object.insert(
                        "deleted_at".to_string(),
                        serde_json::Value::String(timestamp.clone()),
                    );
                    object.insert(
                        "updated_at".to_string(),
                        serde_json::Value::String(timestamp),
                    );
                    Some(serde_json::Value::Object(object))
                }
                None => None,
            };
            items.push(SyncQueueItem {
                identifier: row.try_get("", "identifier").map_err(Self::db_error)?,
                table_name,
                record_identifier,
                operation,
                workspace_identifier: row
                    .try_get("", "workspace_identifier")
                    .map_err(Self::db_error)?,
                created_at,
                attempts: row.try_get("", "attempts").map_err(Self::db_error)?,
                last_error: row.try_get("", "last_error").map_err(Self::db_error)?,
                device_id: row.try_get("", "device_id").map_err(Self::db_error)?,
                payload,
            });
        }
        Ok(items)
    }

    async fn row_payload(
        &self,
        table: &str,
        record_identifier: &str,
    ) -> Result<Option<serde_json::Value>, KernelError> {
        if !SYNCABLE_TABLES.contains(&table) {
            return Ok(None);
        }

        let columns = self.columns(table).await?;
        let json_columns = ["categories", "payload"];
        let timestamp_columns = ["created_at", "updated_at", "deleted_at", "remind_at"];
        let fields = columns
            .iter()
            .map(|column| {
                let expression = if column == "identifier" || column == "workspace_identifier" {
                    format!("lower(hex(\"{column}\"))")
                } else if json_columns.contains(&column.as_str()) {
                    format!("CASE WHEN \"{column}\" IS NULL THEN NULL ELSE json(\"{column}\") END")
                } else if timestamp_columns.contains(&column.as_str()) {
                    format!("strftime('%Y-%m-%dT%H:%M:%fZ', \"{column}\")")
                } else {
                    format!("\"{column}\"")
                };
                format!("'{column}', {expression}")
            })
            .collect::<Vec<_>>()
            .join(", ");

        let sql = format!(
            "SELECT json_object({fields}) AS payload FROM \"{table}\" WHERE lower(hex(identifier)) = ?"
        );
        let stmt = Statement::from_sql_and_values(
            self.backend(),
            sql,
            [Value::String(Some(record_identifier.to_string()))],
        );
        let Some(row) = self
            .conn
            .query_one_raw(stmt)
            .await
            .map_err(Self::db_error)?
        else {
            return Ok(None);
        };
        let payload: Option<String> = row.try_get("", "payload").map_err(Self::db_error)?;
        Ok(payload.and_then(|payload| serde_json::from_str(&payload).ok()))
    }

    pub async fn ack(&self, identifiers: Vec<String>) -> Result<(), KernelError> {
        if identifiers.is_empty() {
            return Ok(());
        }

        let placeholders = identifiers
            .iter()
            .enumerate()
            .map(|(index, _)| format!("lower(hex(identifier)) = {}", self.placeholder(index + 1)))
            .collect::<Vec<_>>()
            .join(" OR ");
        let values = identifiers
            .into_iter()
            .map(|identifier| Value::String(Some(identifier)))
            .collect::<Vec<_>>();
        let stmt = Statement::from_sql_and_values(
            self.backend(),
            format!("DELETE FROM sync_queue WHERE {placeholders}"),
            values,
        );
        self.conn.execute_raw(stmt).await.map_err(Self::db_error)?;
        Ok(())
    }

    pub async fn fail(&self, identifier: String, error: String) -> Result<(), KernelError> {
        let stmt = Statement::from_sql_and_values(
            self.backend(),
            r#"
                UPDATE sync_queue
                SET attempts = attempts + 1, last_error = ?
                WHERE lower(hex(identifier)) = ?
            "#,
            [Value::String(Some(error)), Value::String(Some(identifier))],
        );
        self.conn.execute_raw(stmt).await.map_err(Self::db_error)?;
        Ok(())
    }

    pub async fn state(&self, workspace_identifier: Uuid) -> Result<SyncState, KernelError> {
        let stmt = Statement::from_sql_and_values(
            self.backend(),
            r#"
                SELECT workspace_identifier, last_cursor, last_synced_at, last_error
                FROM sync_state
                WHERE workspace_identifier = ?
            "#,
            [Value::Uuid(Some(workspace_identifier))],
        );
        let row = self
            .conn
            .query_one_raw(stmt)
            .await
            .map_err(Self::db_error)?;

        match row {
            Some(row) => Ok(SyncState {
                workspace_identifier: row
                    .try_get("", "workspace_identifier")
                    .map_err(Self::db_error)?,
                last_cursor: row
                    .try_get::<i64>("", "last_cursor")
                    .map_err(Self::db_error)?
                    .to_string(),
                last_synced_at: row.try_get("", "last_synced_at").map_err(Self::db_error)?,
                last_error: row.try_get("", "last_error").map_err(Self::db_error)?,
            }),
            None => Ok(SyncState {
                workspace_identifier,
                last_cursor: "0".to_string(),
                last_synced_at: None,
                last_error: None,
            }),
        }
    }

    pub async fn set_state(
        &self,
        workspace_identifier: Uuid,
        cursor: String,
        synced_at: Option<String>,
        error: Option<String>,
    ) -> Result<(), KernelError> {
        let cursor = cursor.parse::<i64>().unwrap_or(0);
        let sql = match self.backend() {
            DbBackend::Postgres => {
                r#"
                    INSERT INTO sync_state
                        (workspace_identifier, last_cursor, last_synced_at, last_error)
                    VALUES ($1, $2, $3, $4)
                    ON CONFLICT (workspace_identifier) DO UPDATE SET
                        last_cursor = EXCLUDED.last_cursor,
                        last_synced_at = EXCLUDED.last_synced_at,
                        last_error = EXCLUDED.last_error
                "#
            }
            _ => {
                r#"
                    INSERT INTO sync_state
                        (workspace_identifier, last_cursor, last_synced_at, last_error)
                    VALUES (?, ?, ?, ?)
                    ON CONFLICT (workspace_identifier) DO UPDATE SET
                        last_cursor = excluded.last_cursor,
                        last_synced_at = excluded.last_synced_at,
                        last_error = excluded.last_error
                "#
            }
        };
        let stmt = Statement::from_sql_and_values(
            self.backend(),
            sql,
            [
                Value::Uuid(Some(workspace_identifier)),
                Value::BigInt(Some(cursor)),
                Value::String(synced_at),
                Value::String(error),
            ],
        );
        self.conn.execute_raw(stmt).await.map_err(Self::db_error)?;
        Ok(())
    }

    pub async fn set_applying(&self, applying: bool) -> Result<(), KernelError> {
        let (sql, values) = if applying {
            match self.backend() {
                DbBackend::Postgres => (
                    r#"
                        INSERT INTO sync_meta (key, value)
                        VALUES ('applying', $1)
                        ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value
                    "#,
                    vec![Value::String(Some("1".to_string()))],
                ),
                _ => (
                    r#"
                        INSERT INTO sync_meta (key, value)
                        VALUES ('applying', ?)
                        ON CONFLICT (key) DO UPDATE SET value = excluded.value
                    "#,
                    vec![Value::String(Some("1".to_string()))],
                ),
            }
        } else {
            match self.backend() {
                DbBackend::Postgres => (
                    "DELETE FROM sync_meta WHERE key = $1",
                    vec![Value::String(Some("applying".to_string()))],
                ),
                _ => (
                    "DELETE FROM sync_meta WHERE key = ?",
                    vec![Value::String(Some("applying".to_string()))],
                ),
            }
        };
        let stmt = Statement::from_sql_and_values(self.backend(), sql, values);
        self.conn.execute_raw(stmt).await.map_err(Self::db_error)?;
        Ok(())
    }

    pub async fn device_id(&self) -> Result<String, KernelError> {
        let stmt = Statement::from_sql_and_values(
            self.backend(),
            "SELECT value FROM sync_meta WHERE key = ?",
            [Value::String(Some(DEVICE_KEY.to_string()))],
        );
        if let Some(row) = self
            .conn
            .query_one_raw(stmt)
            .await
            .map_err(Self::db_error)?
        {
            if let Ok(value) = row.try_get::<String>("", "value") {
                if !value.is_empty() {
                    return Ok(value);
                }
            }
        }

        let device_id = Uuid::new_v4().to_string();
        let sql = match self.backend() {
            DbBackend::Postgres => {
                "INSERT INTO sync_meta (key, value) VALUES ($1, $2)
                 ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value"
            }
            _ => {
                "INSERT INTO sync_meta (key, value) VALUES (?, ?)
                 ON CONFLICT (key) DO UPDATE SET value = excluded.value"
            }
        };
        let stmt = Statement::from_sql_and_values(
            self.backend(),
            sql,
            [
                Value::String(Some(DEVICE_KEY.to_string())),
                Value::String(Some(device_id.clone())),
            ],
        );
        self.conn.execute_raw(stmt).await.map_err(Self::db_error)?;
        Ok(device_id)
    }

    async fn columns(&self, table: &str) -> Result<HashSet<String>, KernelError> {
        let sql = match self.backend() {
            DbBackend::Postgres => format!(
                "SELECT column_name AS name FROM information_schema.columns WHERE table_name = '{}'",
                table.replace('\'', "''")
            ),
            _ => format!("PRAGMA table_info(\"{}\")", table.replace('"', "\"\"")),
        };
        let stmt = Statement::from_string(self.backend(), sql);
        let rows = self
            .conn
            .query_all_raw(stmt)
            .await
            .map_err(Self::db_error)?;
        let mut columns = HashSet::new();
        for row in rows {
            if let Ok(name) = row.try_get::<String>("", "name") {
                columns.insert(name);
            }
        }
        Ok(columns)
    }

    fn json_to_value(value: &serde_json::Value) -> Value {
        match value {
            serde_json::Value::Null => Value::String(None),
            serde_json::Value::Bool(value) => Value::Bool(Some(*value)),
            serde_json::Value::Number(value) => {
                if let Some(value) = value.as_i64() {
                    Value::BigInt(Some(value))
                } else if let Some(value) = value.as_u64() {
                    Value::BigInt(Some(value as i64))
                } else {
                    Value::Double(value.as_f64())
                }
            }
            serde_json::Value::String(value) => Value::String(Some(value.clone())),
            other => Value::Json(Some(Box::new(other.clone()))),
        }
    }

    pub async fn apply_changes(
        &self,
        workspace_identifier: Uuid,
        changes: Vec<RemoteSyncChange>,
    ) -> Result<ApplyChangesResult, KernelError> {
        if changes.is_empty() {
            return Ok(ApplyChangesResult::default());
        }

        self.set_applying(true).await?;
        let mut result = ApplyChangesResult::default();

        let mut ordered = changes;
        ordered.sort_by_key(|change| change.table_name != "workspaces");

        for change in ordered {
            match self.apply_change(workspace_identifier, &change).await {
                Ok(()) => result.applied += 1,
                Err(error) => result.errors.push(format!(
                    "{} {}: {}",
                    change.table_name, change.record_identifier, error
                )),
            }
        }

        self.set_applying(false).await?;
        Ok(result)
    }

    async fn apply_change(
        &self,
        workspace_identifier: Uuid,
        change: &RemoteSyncChange,
    ) -> Result<(), KernelError> {
        match change.operation.as_str() {
            "DELETE" => {
                if !SYNCABLE_TABLES.contains(&change.table_name.as_str()) {
                    return Err(KernelError::DbOperationError(format!(
                        "unknown sync table: {}",
                        change.table_name
                    )));
                }

                let timestamp = change.tombstone_timestamp();
                let is_workspace = change.table_name == "workspaces";
                let guarded = if is_workspace {
                    match self.backend() {
                        DbBackend::Postgres => r#"UPDATE "workspaces" SET deleted_at = $2::timestamptz, updated_at = $2::timestamptz, is_hidden = TRUE WHERE identifier = $1 AND (updated_at IS NULL OR updated_at <= $2::timestamptz)"#.to_string(),
                        _ => r#"UPDATE "workspaces" SET deleted_at = ?, updated_at = ?, is_hidden = 1 WHERE identifier = ? AND (updated_at IS NULL OR julianday(updated_at) <= julianday(?))"#.to_string(),
                    }
                } else {
                    match self.backend() {
                        DbBackend::Postgres => format!(
                            r#"UPDATE "{}" SET deleted_at = $2::timestamptz, updated_at = $2::timestamptz WHERE identifier = $1 AND (updated_at IS NULL OR updated_at <= $2::timestamptz)"#,
                            change.table_name
                        ),
                        _ => format!(
                            r#"UPDATE "{}" SET deleted_at = ?, updated_at = ? WHERE identifier = ? AND (updated_at IS NULL OR julianday(updated_at) <= julianday(?))"#,
                            change.table_name
                        ),
                    }
                };
                let values = match self.backend() {
                    DbBackend::Postgres => vec![
                        Value::Uuid(Some(change.record_identifier)),
                        Value::String(Some(timestamp.clone())),
                    ],
                    _ => vec![
                        Value::String(Some(timestamp.clone())),
                        Value::String(Some(timestamp.clone())),
                        Value::Uuid(Some(change.record_identifier)),
                        Value::String(Some(timestamp.clone())),
                    ],
                };
                let result = self
                    .conn
                    .execute_raw(Statement::from_sql_and_values(self.backend(), guarded, values))
                    .await
                    .map_err(Self::db_error)?;

                if result.rows_affected() == 0 {
                    if self
                        .row_exists(&change.table_name, change.record_identifier)
                        .await?
                    {
                        // A local row newer than this delete wins; leave it alone.
                        return Ok(());
                    }
                    let has_content = change
                        .payload
                        .as_ref()
                        .and_then(|value| value.as_object())
                        .map(|object| object.len() > 1)
                        .unwrap_or(false);
                    if !has_content {
                        return Ok(());
                    }
                    // Row was hard-deleted locally before this pull. Recreate it
                    // as a tombstone so stale updates cannot resurrect it.
                    let tombstone = tombstone_payload(
                        change,
                        &timestamp,
                        workspace_identifier,
                        is_workspace,
                    );
                    self.upsert_payload(
                        &change.table_name,
                        change.record_identifier,
                        &tombstone,
                    )
                    .await?;
                    if !is_workspace {
                        self.stage_recycle_bin(&change.table_name, &tombstone)
                            .await?;
                    }
                } else if !is_workspace {
                    let tombstone = tombstone_payload(
                        change,
                        &timestamp,
                        workspace_identifier,
                        is_workspace,
                    );
                    self.stage_recycle_bin(&change.table_name, &tombstone)
                        .await?;
                }

                if is_workspace {
                    self.ensure_default_workspace().await?;
                }
                Ok(())
            }
            "INSERT" | "UPDATE" => {
                let payload = change
                    .payload
                    .as_ref()
                    .ok_or_else(|| {
                        KernelError::DbOperationError("sync change is missing a payload".into())
                    })?
                    .clone();
                let mut object = payload.as_object().cloned().ok_or_else(|| {
                    KernelError::DbOperationError("sync change payload must be an object".into())
                })?;
                if change.table_name != "workspaces" {
                    object.insert(
                        "workspace_identifier".to_string(),
                        serde_json::Value::String(workspace_identifier.to_string()),
                    );
                }
                let payload = serde_json::Value::Object(object);

                self.upsert_payload(
                    &change.table_name,
                    change.record_identifier,
                    &payload,
                )
                .await?;
                self.stage_recycle_bin(&change.table_name, &payload).await?;
                Ok(())
            }
            other => Err(KernelError::DbOperationError(format!(
                "unsupported sync operation: {other}"
            ))),
        }
    }

    async fn upsert_payload(
        &self,
        table_name: &str,
        record_identifier: Uuid,
        payload: &serde_json::Value,
    ) -> Result<(), KernelError> {
        let object = payload.as_object().ok_or_else(|| {
            KernelError::DbOperationError("sync change payload must be an object".into())
        })?;
        let allowed = self.columns(table_name).await?;
        if allowed.is_empty() {
            return Err(KernelError::DbOperationError(format!(
                "unknown sync table: {table_name}"
            )));
        }

        let mut columns = Vec::new();
        let mut values = Vec::new();
        for (key, value) in object {
            if !allowed.contains(key) {
                continue;
            }
            columns.push(key.clone());
            values.push(Self::json_to_value(value));
        }
        if !columns.iter().any(|column| column == "identifier") {
            columns.push("identifier".to_string());
            values.push(Value::Uuid(Some(record_identifier)));
        }

        let placeholders = (0..columns.len())
            .map(|index| self.placeholder(index + 1))
            .collect::<Vec<_>>()
            .join(", ");
        let quoted = columns
            .iter()
            .map(|column| format!("\"{column}\""))
            .collect::<Vec<_>>()
            .join(", ");
        let updates = columns
            .iter()
            .filter(|column| column.as_str() != "identifier")
            .map(|column| format!("\"{column}\" = excluded.\"{column}\""))
            .collect::<Vec<_>>()
            .join(", ");

        let sql = if updates.is_empty() {
            format!(
                "INSERT INTO \"{table_name}\" ({quoted}) VALUES ({placeholders}) ON CONFLICT (identifier) DO NOTHING"
            )
        } else {
            format!(
                "INSERT INTO \"{table_name}\" ({quoted}) VALUES ({placeholders}) \
                 ON CONFLICT (identifier) DO UPDATE SET {updates} \
                 WHERE excluded.\"updated_at\" >= \"{table_name}\".\"updated_at\""
            )
        };

        let stmt = Statement::from_sql_and_values(self.backend(), sql, values);
        self.conn.execute_raw(stmt).await.map_err(Self::db_error)?;
        Ok(())
    }

    async fn row_exists(
        &self,
        table_name: &str,
        record_identifier: Uuid,
    ) -> Result<bool, KernelError> {
        let sql = format!(
            "SELECT 1 AS present FROM \"{table_name}\" WHERE identifier = {}",
            self.placeholder(1)
        );
        let stmt = Statement::from_sql_and_values(
            self.backend(),
            sql,
            [Value::Uuid(Some(record_identifier))],
        );
        Ok(self
            .conn
            .query_one_raw(stmt)
            .await
            .map_err(Self::db_error)?
            .is_some())
    }

    async fn ensure_default_workspace(&self) -> Result<(), KernelError> {
        let sql = match self.backend() {
            DbBackend::Postgres => r#"UPDATE "workspaces" SET is_default = TRUE WHERE identifier = (SELECT identifier FROM "workspaces" WHERE deleted_at IS NULL ORDER BY created_at ASC LIMIT 1) AND NOT EXISTS (SELECT 1 FROM "workspaces" WHERE deleted_at IS NULL AND is_default = TRUE)"#,
            _ => r#"UPDATE "workspaces" SET is_default = 1 WHERE identifier = (SELECT identifier FROM "workspaces" WHERE deleted_at IS NULL ORDER BY created_at ASC LIMIT 1) AND NOT EXISTS (SELECT 1 FROM "workspaces" WHERE deleted_at IS NULL AND is_default = 1)"#,
        };
        self.conn
            .execute_raw(Statement::from_string(self.backend(), sql.to_string()))
            .await
            .map_err(Self::db_error)?;
        Ok(())
    }

    async fn stage_recycle_bin(
        &self,
        table_name: &str,
        payload: &serde_json::Value,
    ) -> Result<(), KernelError> {
        let Some(item_type) = RECYCLE_BIN_TABLES
            .iter()
            .find(|(table, _)| *table == table_name)
            .map(|(_, item_type)| *item_type)
        else {
            return Ok(());
        };
        let Some(identifier) = payload
            .get("identifier")
            .and_then(|value| value.as_str())
            .and_then(|value| Uuid::parse_str(value).ok())
        else {
            return Ok(());
        };

        let tombstone_at = match payload.get("deleted_at") {
            Some(serde_json::Value::Null) | None => None,
            Some(value) => value.as_str().map(str::to_string),
        };
        let Some(tombstone_at) = tombstone_at else {
            self.conn
                .execute_raw(Statement::from_sql_and_values(
                    self.backend(),
                    "DELETE FROM recycle_bin WHERE item_id = ? AND item_type = ?",
                    [
                        Value::Uuid(Some(identifier)),
                        Value::String(Some(item_type.to_string())),
                    ],
                ))
                .await
                .map_err(Self::db_error)?;
            return Ok(());
        };

        let workspace_identifier = payload
            .get("workspace_identifier")
            .and_then(|value| value.as_str())
            .and_then(|value| Uuid::parse_str(value).ok());
        let Some(workspace_identifier) = workspace_identifier else {
            return Ok(());
        };

        let update_sql = match self.backend() {
            DbBackend::Postgres => {
                r#"UPDATE recycle_bin SET payload = $1, deleted_at = $2, workspace_identifier = $3 WHERE item_id = $4 AND item_type = $5"#
            }
            _ => {
                r#"UPDATE recycle_bin SET payload = ?, deleted_at = ?, workspace_identifier = ? WHERE item_id = ? AND item_type = ?"#
            }
        };
        let updated = self
            .conn
            .execute_raw(Statement::from_sql_and_values(
                self.backend(),
                update_sql,
                [
                    Value::String(Some(payload.to_string())),
                    Value::String(Some(tombstone_at.clone())),
                    Value::Uuid(Some(workspace_identifier)),
                    Value::Uuid(Some(identifier)),
                    Value::String(Some(item_type.to_string())),
                ],
            ))
            .await
            .map_err(Self::db_error)?;
        if updated.rows_affected() > 0 {
            return Ok(());
        }

        let sql = match self.backend() {
            DbBackend::Postgres => {
                r#"
                    INSERT INTO recycle_bin
                        (identifier, item_id, item_type, payload, deleted_at, workspace_identifier)
                    VALUES ($1, $2, $3, $4, $5, $6)
                "#
            }
            _ => {
                r#"
                    INSERT INTO recycle_bin
                        (identifier, item_id, item_type, payload, deleted_at, workspace_identifier)
                    VALUES (?, ?, ?, ?, ?, ?)
                "#
            }
        };
        let stmt = Statement::from_sql_and_values(
            self.backend(),
            sql,
            [
                Value::Uuid(Some(Uuid::new_v4())),
                Value::Uuid(Some(identifier)),
                Value::String(Some(item_type.to_string())),
                Value::String(Some(payload.to_string())),
                Value::String(Some(tombstone_at)),
                Value::Uuid(Some(workspace_identifier)),
            ],
        );
        self.conn.execute_raw(stmt).await.map_err(Self::db_error)?;
        Ok(())
    }

    pub async fn clear_applying(&self) -> Result<(), KernelError> {
        self.set_applying(false).await
    }
}

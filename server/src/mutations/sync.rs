use std::collections::HashMap;
use std::sync::Arc;

use chrono::Utc;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement, Value};
use seaography::{
    async_graphql::{self, Context},
    CustomFields, CustomInputType, CustomOutputType,
};
use serde::{Deserialize, Serialize};

use crate::{
    adapters::jwt::Claims,
    services::workspace_member_service::WorkspaceMemberService,
    utils::context::{extract_claims, extract_db_conn},
};

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

#[derive(CustomInputType, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[seaography(input_type_name = "SyncChangeInput")]
pub struct SyncChangeInput {
    pub table_name: String,
    pub record_identifier: String,
    pub operation: String,
    pub payload: serde_json::Value,
    #[serde(default)]
    pub deleted_at: Option<String>,
}

#[derive(Clone, CustomOutputType)]
#[seaography(output_type_name = "SyncChangeOutput")]
pub struct SyncChangeOutput {
    pub seq: String,
    pub table_name: String,
    pub record_identifier: Option<String>,
    pub operation: String,
    pub payload: Option<serde_json::Value>,
    pub updated_at: String,
}

#[derive(Clone, CustomOutputType)]
#[seaography(output_type_name = "SyncPushResult")]
pub struct SyncPushResult {
    pub applied: Vec<String>,
    pub errors: Vec<String>,
}

#[derive(Clone, CustomOutputType)]
#[seaography(output_type_name = "SyncPullResult")]
pub struct SyncPullResult {
    pub changes: Vec<SyncChangeOutput>,
    pub next_cursor: String,
    pub has_more: bool,
}

pub struct SyncMutation;

//TODO use camelCase 
#[CustomFields]
impl SyncMutation {
    async fn sync_push(
        ctx: &Context<'_>,
        workspace_identifier: String,
        _device_id: Option<String>,
        changes: Vec<SyncChangeInput>,
    ) -> async_graphql::Result<SyncPushResult> {
        let (claims, claim_workspace) = authorize_sync(ctx, workspace_identifier.clone()).await?;
        let db = extract_db_conn(ctx)?;

        let mut ordered = changes;
        ordered.sort_by_key(|change| (change.table_name != "workspaces", 0));

        let mut applied = Vec::new();
        let mut errors = Vec::new();
        let mut columns: HashMap<String, Vec<String>> = HashMap::new();

        for change in ordered {
            if !SYNCABLE_TABLES.contains(&change.table_name.as_str()) {
                errors.push(format!("unsupported table: {}", change.table_name));
                continue;
            }

            let result = apply_change(db, workspace_identifier.clone(), &change, &mut columns).await;

            match result {
                Ok(true) => applied.push(change.record_identifier.to_string()),
                Ok(false) => {}
                Err(error) => errors.push(format!(
                    "{} {}: {error}",
                    change.table_name, change.record_identifier
                )),
            }
        }

        if claim_workspace {
            claim_workspace_if_ownerless(db, workspace_identifier, &claims).await?;
        }

        Ok(SyncPushResult { applied, errors })
    }
}

pub struct SyncQuery;

#[CustomFields]
impl SyncQuery {
    async fn sync_pull(
        ctx: &Context<'_>,
        workspace_identifier: String,
        since: String,
        limit: Option<i32>,
    ) -> async_graphql::Result<SyncPullResult> {
        let (claims, claim_workspace) = authorize_sync(ctx, workspace_identifier.clone()).await?;
        let db = extract_db_conn(ctx)?;

        let since = since
            .parse::<i64>()
            .map_err(|_| async_graphql::Error::new("invalid since cursor"))?;
        let limit = limit.unwrap_or(500).clamp(1, 1000);

        let snapshot = db
            .query_one_raw(Statement::from_sql_and_values(
                DbBackend::Postgres,
                "SELECT COALESCE(MAX(seq), $1) AS max_seq FROM sync_log WHERE workspace_identifier = $2",
                vec![Value::BigInt(Some(since)), Value::String(Some(workspace_identifier.clone()))],
            ))
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?
            .map(|row| row.try_get::<i64>("", "max_seq"))
            .transpose()
            .map_err(|e| async_graphql::Error::new(e.to_string()))?
            .unwrap_or(since);

        let rows = db
            .query_all_raw(Statement::from_sql_and_values(
                DbBackend::Postgres,
                "SELECT seq, table_name, record_identifier, operation, payload, created_at
                 FROM sync_log
                 WHERE workspace_identifier = $1 AND seq > $2 AND seq <= $3
                 ORDER BY seq ASC
                 LIMIT $4",
                vec![
                    Value::String(Some(workspace_identifier.clone())),
                    Value::BigInt(Some(since)),
                    Value::BigInt(Some(snapshot)),
                    Value::BigInt(Some(limit as i64)),
                ],
            ))
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let mut changes = Vec::with_capacity(rows.len());
        let mut next_cursor = since;

        for row in rows {
            let seq = row
                .try_get::<i64>("", "seq")
                .map_err(|e| async_graphql::Error::new(e.to_string()))?;
            let table_name = row
                .try_get::<String>("", "table_name")
                .map_err(|e| async_graphql::Error::new(e.to_string()))?;
            let record_identifier = row
                .try_get::<Option<String>>("", "record_identifier")
                .map_err(|e| async_graphql::Error::new(e.to_string()))?;
            let operation = row
                .try_get::<String>("", "operation")
                .map_err(|e| async_graphql::Error::new(e.to_string()))?;
            let payload = row
                .try_get::<Option<serde_json::Value>>("", "payload")
                .map_err(|e| async_graphql::Error::new(e.to_string()))?;
            let created_at = row
                .try_get::<chrono::DateTime<chrono::FixedOffset>>("", "created_at")
                .map_err(|e| async_graphql::Error::new(e.to_string()))?;

            let updated_at = payload
                .as_ref()
                .and_then(|value| value.get("updated_at"))
                .and_then(|value| value.as_str())
                .map(str::to_string)
                .unwrap_or_else(|| created_at.to_rfc3339());

            next_cursor = seq;
            changes.push(SyncChangeOutput {
                seq: seq.to_string(),
                table_name,
                record_identifier,
                operation,
                payload,
                updated_at,
            });
        }

        let has_more = changes.len() == limit as usize;

        if claim_workspace {
            claim_workspace_if_ownerless(db, workspace_identifier, &claims).await?;
        }

        Ok(SyncPullResult {
            changes,
            next_cursor: next_cursor.to_string(),
            has_more,
        })
    }

    async fn sync_head(
        ctx: &Context<'_>,
        workspace_identifier: String,
    ) -> async_graphql::Result<String> {
        let _ = authorize_sync(ctx, workspace_identifier.clone()).await?;
        let db = extract_db_conn(ctx)?;

        let row = db
            .query_one_raw(Statement::from_sql_and_values(
                DbBackend::Postgres,
                "SELECT COALESCE(MAX(seq), 0) AS max_seq FROM sync_log WHERE workspace_identifier = $1",
                vec![Value::String(Some(workspace_identifier.clone()))],
            ))
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let max_seq = row
            .map(|row| row.try_get::<i64>("", "max_seq"))
            .transpose()
            .map_err(|e| async_graphql::Error::new(e.to_string()))?
            .unwrap_or(0);

        Ok(max_seq.to_string())
    }
}

async fn authorize_sync(
    ctx: &Context<'_>,
    workspace_identifier: String,
) -> async_graphql::Result<(Claims, bool)> {
    let claims = extract_claims(ctx)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
    let db = extract_db_conn(ctx)?;
    let members = WorkspaceMemberService::init(&Arc::new(db.clone()));

    let existing = members
        .list_members(workspace_identifier.clone())
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

    if existing.is_empty() {
        return Ok((claims, true));
    }

    members
        .assert_member(workspace_identifier.clone(), &claims.email)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

    Ok((claims, false))
}

async fn claim_workspace_if_ownerless(
    db: &DatabaseConnection,
    workspace_identifier: String,
    claims: &Claims,
) -> async_graphql::Result<()> {
    let exists = db
        .query_one_raw(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT 1 AS present FROM workspaces WHERE identifier = $1 AND deleted_at IS NULL",
            vec![Value::String(Some(workspace_identifier.clone()))],
        ))
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?
        .is_some();

    if !exists {
        return Ok(());
    }

    let members = WorkspaceMemberService::init(&Arc::new(db.clone()));
    let existing = members
        .list_members(workspace_identifier.clone())
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

    if existing.is_empty() {
        members
            .ensure_owner(workspace_identifier.clone(), claims)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
    }

    Ok(())
}

async fn apply_change(
    db: &DatabaseConnection,
    workspace_identifier: String,
    change: &SyncChangeInput,
    columns: &mut HashMap<String, Vec<String>>,
) -> Result<bool, String> {
    let mut payload = change.payload.clone();

    if let Some(object) = payload.as_object_mut() {
        object.insert(
            "identifier".to_string(),
            serde_json::Value::String(change.record_identifier.to_string()),
        );
        if change.table_name != "workspaces" {
            object.insert(
                "workspace_identifier".to_string(),
                serde_json::Value::String(workspace_identifier.to_string()),
            );
        }
    } else {
        return Err("payload must be an object".to_string());
    }

    if change.operation == "DELETE" {
        let timestamp = change
            .deleted_at
            .clone()
            .or_else(|| {
                payload
                    .get("deleted_at")
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
            })
            .or_else(|| {
                payload
                    .get("updated_at")
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
            })
            .unwrap_or_else(|| Utc::now().to_rfc3339());

        if change.table_name == "workspaces" {
            db.execute_raw(Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"UPDATE "workspaces"
                   SET "deleted_at" = $2::timestamptz,
                       "updated_at" = $2::timestamptz,
                       "is_hidden" = TRUE
                   WHERE "identifier" = $1
                     AND ("updated_at" IS NULL OR "updated_at" <= $2::timestamptz)"#,
                vec![
                    Value::String(Some(change.record_identifier.clone())),
                    Value::String(Some(timestamp)),
                ],
            ))
            .await
            .map_err(|e| e.to_string())?;

            db.execute_raw(Statement::from_string(
                DbBackend::Postgres,
                r#"UPDATE "workspaces" SET "is_default" = TRUE
                   WHERE "identifier" = (
                       SELECT "identifier" FROM "workspaces"
                       WHERE "deleted_at" IS NULL ORDER BY "created_at" ASC LIMIT 1
                   )
                   AND NOT EXISTS (
                       SELECT 1 FROM "workspaces"
                       WHERE "deleted_at" IS NULL AND "is_default" = TRUE
                   )"#
                .to_string(),
            ))
            .await
            .map_err(|e| e.to_string())?;
        } else {
            db.execute_raw(Statement::from_sql_and_values(
                DbBackend::Postgres,
                &format!(
                    r#"UPDATE "{}"
                       SET "deleted_at" = $3::timestamptz,
                           "updated_at" = $3::timestamptz
                       WHERE "identifier" = $1
                         AND "workspace_identifier" = $2
                         AND ("updated_at" IS NULL OR "updated_at" <= $3::timestamptz)"#,
                    change.table_name
                ),
                vec![
                    Value::String(Some(change.record_identifier.clone())),
                    Value::String(Some(workspace_identifier.clone())),
                    Value::String(Some(timestamp)),
                ],
            ))
            .await
            .map_err(|e| e.to_string())?;
        }
        return Ok(true);
    }

    if change.operation != "INSERT" && change.operation != "UPDATE" {
        return Err(format!("unsupported operation: {}", change.operation));
    }

    let cols = columns_for(db, &change.table_name, columns).await?;
    let quoted: Vec<String> = cols.iter().map(|col| format!(r#""{col}""#)).collect();
    let assignments: Vec<String> = quoted
        .iter()
        .filter(|col| col.as_str() != r#""identifier""#)
        .map(|col| format!(r#"{col} = EXCLUDED.{col}"#))
        .collect();

    let sql = format!(
        r#"INSERT INTO "{}" ({})
           SELECT {} FROM jsonb_populate_record(NULL::"{}", $1::jsonb)
           ON CONFLICT ("identifier") DO UPDATE SET {}
           WHERE "{}"."updated_at" <= EXCLUDED."updated_at""#,
        change.table_name,
        quoted.join(", "),
        quoted.join(", "),
        change.table_name,
        assignments.join(", "),
        change.table_name,
    );

    db.execute_raw(Statement::from_sql_and_values(
        DbBackend::Postgres,
        &sql,
        vec![Value::Json(Some(Box::new(payload)))],
    ))
    .await
    .map_err(|e| e.to_string())?;

    Ok(true)
}

async fn columns_for(
    db: &DatabaseConnection,
    table: &str,
    cache: &mut HashMap<String, Vec<String>>,
) -> Result<Vec<String>, String> {
    if let Some(cols) = cache.get(table) {
        return Ok(cols.clone());
    }

    let rows = db
        .query_all_raw(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT column_name FROM information_schema.columns
             WHERE table_schema = 'public' AND table_name = $1 AND is_generated = 'NEVER'
             ORDER BY ordinal_position",
            vec![Value::String(Some(table.to_string()))],
        ))
        .await
        .map_err(|e| e.to_string())?;

    let cols: Vec<String> = rows
        .into_iter()
        .filter_map(|row| row.try_get::<String>("", "column_name").ok())
        .collect();

    if cols.is_empty() {
        return Err(format!("unknown table: {table}"));
    }

    cache.insert(table.to_string(), cols.clone());
    Ok(cols)
}

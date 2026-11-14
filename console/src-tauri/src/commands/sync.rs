use kernel::repositories::sync::{ApplyChangesResult, RemoteSyncChange, SyncQueueItem, SyncState};
use tauri::State;
use uuid::Uuid;

use crate::{errors::AppError, state::app::AppState};

#[tauri::command]
pub async fn sync_queue_list(
    state: State<'_, AppState>,
    workspace_identifier: Uuid,
) -> Result<Vec<SyncQueueItem>, AppError> {
    state
        .sync_repository
        .queue(workspace_identifier)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn sync_queue_ack(
    state: State<'_, AppState>,
    identifiers: Vec<String>,
) -> Result<(), AppError> {
    state
        .sync_repository
        .ack(identifiers)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn sync_queue_fail(
    state: State<'_, AppState>,
    identifier: String,
    error: String,
) -> Result<(), AppError> {
    state
        .sync_repository
        .fail(identifier, error)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn sync_state_get(
    state: State<'_, AppState>,
    workspace_identifier: Uuid,
) -> Result<SyncState, AppError> {
    state
        .sync_repository
        .state(workspace_identifier)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn sync_state_set(
    state: State<'_, AppState>,
    workspace_identifier: Uuid,
    cursor: String,
    synced_at: Option<String>,
    error: Option<String>,
) -> Result<(), AppError> {
    state
        .sync_repository
        .set_state(workspace_identifier, cursor, synced_at, error)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn sync_apply_changes(
    state: State<'_, AppState>,
    workspace_identifier: Uuid,
    changes: Vec<RemoteSyncChange>,
) -> Result<ApplyChangesResult, AppError> {
    state
        .sync_repository
        .apply_changes(workspace_identifier, changes)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn sync_set_applying(state: State<'_, AppState>, applying: bool) -> Result<(), AppError> {
    state
        .sync_repository
        .set_applying(applying)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn sync_clear_applying(state: State<'_, AppState>) -> Result<(), AppError> {
    state
        .sync_repository
        .clear_applying()
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn sync_device_id(state: State<'_, AppState>) -> Result<String, AppError> {
    state.sync_repository.device_id().await.map_err(Into::into)
}

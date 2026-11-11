use std::path::PathBuf;

use kernel::{
    adapters::meta::RequestMeta,
    entities::whiteboards,
    repositories::whiteboards::WhiteboardRepositoryExt,
    repositories::workspace_manager::TransferRecord,
};
use serde::Serialize;
use tauri::{AppHandle, Manager, State};
use uuid::Uuid;

use crate::{
    adapters::whiteboards::{CreateWhiteboard, UpdateWhiteboard},
    errors::AppError,
    state::app::AppState,
};

// ---------------------------------------------------------------------------
// Scene commands (SQLite via the kernel repository)
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn create_whiteboard(
    state: State<'_, AppState>,
    whiteboard: CreateWhiteboard,
    meta: Option<RequestMeta>,
) -> Result<whiteboards::Model, AppError> {
    let created = state
        .whiteboards_repository
        .create(&whiteboard.into(), &meta)
        .await?;
    Ok(created)
}

#[tauri::command]
pub async fn get_whiteboard(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<Option<whiteboards::Model>, AppError> {
    state
        .whiteboards_repository
        .find_by_id(&identifier, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_all_whiteboards(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<whiteboards::Model>, AppError> {
    state
        .whiteboards_repository
        .find_all(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_recently_added_whiteboards(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<whiteboards::Model>, AppError> {
    state
        .whiteboards_repository
        .recently_added(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn update_whiteboard(
    state: State<'_, AppState>,
    identifier: Uuid,
    whiteboard: UpdateWhiteboard,
    meta: Option<RequestMeta>,
) -> Result<whiteboards::Model, AppError> {
    let updated = state
        .whiteboards_repository
        .update(&identifier, &whiteboard.into(), &meta)
        .await?;
    Ok(updated)
}

#[tauri::command]
pub async fn delete_whiteboard(
    app: AppHandle,
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .whiteboards_repository
        .delete(&identifier, &meta)
        .await?;

    // Remove the whiteboard's image files (the DB row is already gone).
    if let Ok(dir) = whiteboard_files_dir(&app, &identifier) {
        let _ = std::fs::remove_dir_all(dir);
    }

    Ok(())
}

#[tauri::command]
pub async fn transfer_whiteboard(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .whiteboards_repository
        .transfer_record(
            &record_identifier,
            &previous_workspace_identifier,
            &target_workspace_identifier,
        )
        .await
        .map_err(AppError::from)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// File commands (filesystem — one data-URL text file per image)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WhiteboardFile {
    pub identifier: String,
    pub whiteboard_identifier: String,
    pub mime_type: String,
    pub data: String,
    pub created_at: String,
    pub updated_at: String,
}

const MAX_FILE_BYTES: usize = 20 * 1024 * 1024;

fn whiteboards_root(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    Ok(app_data_dir.join("whiteboard"))
}

fn whiteboard_files_dir(app: &AppHandle, whiteboard_id: &Uuid) -> Result<PathBuf, String> {
    Ok(whiteboards_root(app)?.join(whiteboard_id.to_string()))
}

fn file_path(app: &AppHandle, whiteboard_id: &Uuid, file_id: &Uuid) -> Result<PathBuf, String> {
    Ok(whiteboard_files_dir(app, whiteboard_id)?.join(file_id.to_string()))
}

/// Parse the mime type out of a `data:<mime>;base64,...` URL.
fn mime_from_data_url(data: &str) -> String {
    data.strip_prefix("data:")
        .and_then(|rest| rest.split(';').next())
        .filter(|mime| !mime.is_empty() && mime.contains('/'))
        .unwrap_or("application/octet-stream")
        .to_string()
}

fn system_time_to_rfc3339(time: Option<std::time::SystemTime>) -> String {
    time.map(|t| chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339())
        .unwrap_or_else(|| chrono::Utc::now().to_rfc3339())
}

#[tauri::command]
pub async fn save_whiteboard_file(
    app: AppHandle,
    whiteboard_identifier: String,
    file_id: String,
    _mime_type: String,
    data: String,
    _meta: Option<RequestMeta>,
) -> Result<WhiteboardFile, String> {
    if data.len() > MAX_FILE_BYTES {
        return Err("file too large (max 20 MB)".into());
    }

    let whiteboard_id = Uuid::parse_str(&whiteboard_identifier)
        .map_err(|_| "invalid whiteboard identifier".to_string())?;
    let file_uuid = Uuid::parse_str(&file_id)
        .map_err(|_| "invalid file identifier".to_string())?;

    let dir = whiteboard_files_dir(&app, &whiteboard_id)?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let path = file_path(&app, &whiteboard_id, &file_uuid)?;
    std::fs::write(&path, &data).map_err(|e| e.to_string())?;

    let now = chrono::Utc::now().to_rfc3339();
    Ok(WhiteboardFile {
        identifier: file_id,
        whiteboard_identifier,
        mime_type: mime_from_data_url(&data),
        data,
        created_at: now.clone(),
        updated_at: now,
    })
}

#[tauri::command]
pub async fn get_whiteboard_files(
    app: AppHandle,
    whiteboard_identifier: String,
    _meta: Option<RequestMeta>,
) -> Result<Vec<WhiteboardFile>, String> {
    let whiteboard_id = Uuid::parse_str(&whiteboard_identifier)
        .map_err(|_| "invalid whiteboard identifier".to_string())?;

    let dir = whiteboard_files_dir(&app, &whiteboard_id)?;

    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut files = vec![];

    let mut entries: Vec<_> = std::fs::read_dir(&dir)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .collect();
    entries.sort_by_key(|e| {
        e.metadata()
            .and_then(|m| m.created().or_else(|_| m.modified()))
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });

    for entry in entries {
        let path = entry.path();
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        // Only accept UUID-named files created by save_whiteboard_file
        if Uuid::parse_str(&file_name).is_err() {
            continue;
        }

        let data = match std::fs::read_to_string(&path) {
            Ok(data) => data,
            Err(_) => continue,
        };

        let metadata = entry.metadata().ok();
        files.push(WhiteboardFile {
            identifier: file_name,
            whiteboard_identifier: whiteboard_identifier.clone(),
            mime_type: mime_from_data_url(&data),
            data,
            created_at: system_time_to_rfc3339(
                metadata.as_ref().and_then(|m| m.created().ok()),
            ),
            updated_at: system_time_to_rfc3339(
                metadata.as_ref().and_then(|m| m.modified().ok()),
            ),
        });
    }

    Ok(files)
}

#[tauri::command]
pub async fn delete_whiteboard_file(
    app: AppHandle,
    file_id: String,
    _meta: Option<RequestMeta>,
) -> Result<(), String> {
    let file_uuid = Uuid::parse_str(&file_id)
        .map_err(|_| "invalid file identifier".to_string())?;

    let root = whiteboards_root(&app)?;
    if !root.exists() {
        return Ok(());
    }

    // The file id is unique; find it under any whiteboard directory.
    for entry in std::fs::read_dir(&root)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
    {
        let path = entry.path().join(file_uuid.to_string());
        if path.is_file() {
            std::fs::remove_file(&path).map_err(|e| e.to_string())?;
            return Ok(());
        }
    }

    Ok(())
}

use std::path::PathBuf;
use std::sync::Arc;

use kernel::{
    adapters::meta::RequestMeta,
    entities::{whiteboard_files, whiteboards},
    filestore::FileStoreProvider,
    repositories::whiteboards::WhiteboardRepositoryExt,
    repositories::workspace_manager::TransferRecord,
    sea_orm::{
        ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    },
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
// File commands (filesystem — one binary image file per image)
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

fn mime_from_data_url(data: &str) -> String {
    data.strip_prefix("data:")
        .and_then(|rest| rest.split(';').next())
        .filter(|mime| !mime.is_empty() && mime.contains('/'))
        .unwrap_or("application/octet-stream")
        .to_string()
}

fn extension_from_mime(mime: &str) -> &str {
    match mime {
        "image/png" => "png",
        "image/jpeg" | "image/jpg" => "jpg",
        "image/gif" => "gif",
        "image/webp" => "webp",
        "image/svg+xml" => "svg",
        _ => "bin",
    }
}

fn mime_from_extension(ext: &str) -> &str {
    match ext {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        _ => "application/octet-stream",
    }
}

fn system_time_to_rfc3339(time: Option<std::time::SystemTime>) -> String {
    time.map(|t| chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339())
        .unwrap_or_else(|| chrono::Utc::now().to_rfc3339())
}

fn decode_data_url(data: &str) -> Option<Vec<u8>> {
    let after_comma = data.find(',').map(|i| &data[i + 1..])?;
    use base64::{Engine, engine::general_purpose::STANDARD};
    STANDARD.decode(after_comma).ok()
}

fn bytes_to_data_url(bytes: &[u8], mime: &str) -> String {
    use base64::{Engine, engine::general_purpose::STANDARD};
    let b64 = STANDARD.encode(bytes);
    format!("data:{mime};base64,{b64}")
}

/// Find a file for a given UUID under a directory, matching `{uuid}.{ext}` or
/// bare `{uuid}` (legacy format).
fn find_file_by_uuid(dir: &std::path::Path, uuid: &Uuid) -> Option<PathBuf> {
    let uuid_str = uuid.to_string();
    std::fs::read_dir(dir).ok()?.filter_map(|e| e.ok()).find_map(|e| {
        let path = e.path();
        if !path.is_file() {
            return None;
        }
        let stem = path.file_stem()?.to_str()?;
        if stem == uuid_str {
            return Some(path);
        }
        None
    })
}

#[tauri::command]
pub async fn save_whiteboard_file(
    app: AppHandle,
    state: State<'_, AppState>,
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

    let mime = mime_from_data_url(&data);
    let ext = extension_from_mime(&mime);
    let bytes = decode_data_url(&data).ok_or("malformed data URL")?;

    let dir = whiteboard_files_dir(&app, &whiteboard_id)?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let path = dir.join(format!("{file_uuid}.{ext}"));
    std::fs::write(&path, &bytes).map_err(|e| e.to_string())?;

    // Background CDN upload — reads directly from the binary file on disk
    let cloud_storage = state.cloud_storage.clone();
    let db = state.db.clone();
    let mime_clone = mime.clone();
    let file_name = format!("{file_uuid}.{ext}");

    tokio::spawn(async move {
        upload_to_cloud(cloud_storage, db, file_uuid, whiteboard_id, &path, &mime_clone, &file_name).await;
    });

    let now = chrono::Utc::now().to_rfc3339();
    Ok(WhiteboardFile {
        identifier: file_id,
        whiteboard_identifier,
        mime_type: mime,
        data,
        created_at: now.clone(),
        updated_at: now,
    })
}

async fn upload_to_cloud(
    cloud_storage: Arc<tokio::sync::RwLock<Option<kernel::filestore::FileStoreEngine>>>,
    db: Arc<DatabaseConnection>,
    file_id: Uuid,
    whiteboard_id: Uuid,
    file_path: &std::path::Path,
    mime: &str,
    file_name: &str,
) {
    let guard = cloud_storage.read().await;
    let engine = match guard.as_ref() {
        Some(e) => e,
        None => return,
    };

    let result = engine.upload_file(file_path, file_name, Some(mime)).await;

    let remote_url = match result {
        Ok(url) => url,
        Err(e) => {
            log::warn!("cloud upload failed for {file_id}: {e}");
            return;
        }
    };

    let now = chrono::Utc::now().into();
    let active = whiteboard_files::ActiveModel {
        identifier: Set(file_id),
        whiteboard_identifier: Set(whiteboard_id),
        mime_type: Set(mime.to_string()),
        data: Set(String::new()),
        remote_url: Set(Some(remote_url.clone())),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let existing = whiteboard_files::Entity::find_by_id(file_id)
        .one(db.as_ref())
        .await;

    match existing {
        Ok(Some(_)) => {
            let update = whiteboard_files::ActiveModel {
                identifier: Set(file_id),
                remote_url: Set(Some(remote_url)),
                updated_at: Set(now),
                ..Default::default()
            };
            if let Err(e) = update.update(db.as_ref()).await {
                log::warn!("failed to update remote_url for {file_id}: {e}");
            }
        }
        _ => {
            if let Err(e) = active.insert(db.as_ref()).await {
                log::warn!("failed to insert whiteboard_files row for {file_id}: {e}");
            }
        }
    }

    log::info!("cloud backup complete for file {file_id}");
}

#[tauri::command]
pub async fn get_whiteboard_files(
    app: AppHandle,
    state: State<'_, AppState>,
    whiteboard_identifier: String,
    _meta: Option<RequestMeta>,
) -> Result<Vec<WhiteboardFile>, String> {
    let whiteboard_id = Uuid::parse_str(&whiteboard_identifier)
        .map_err(|_| "invalid whiteboard identifier".to_string())?;

    let dir = whiteboard_files_dir(&app, &whiteboard_id)?;

    let mut files = vec![];
    let mut local_ids = std::collections::HashSet::new();

    if dir.exists() {
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

            let stem = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            if Uuid::parse_str(&stem).is_err() {
                continue;
            }

            let ext = path.extension().and_then(|e| e.to_str());

            let (data, mime_type) = if let Some(ext) = ext {
                // New format: binary file with extension
                let bytes = match std::fs::read(&path) {
                    Ok(b) => b,
                    Err(_) => continue,
                };
                let mime = mime_from_extension(ext).to_string();
                (bytes_to_data_url(&bytes, &mime), mime)
            } else {
                // Legacy format: raw data-URL text file (no extension)
                let text = match std::fs::read_to_string(&path) {
                    Ok(t) => t,
                    Err(_) => continue,
                };
                let mime = mime_from_data_url(&text);
                (text, mime)
            };

            local_ids.insert(stem.clone());
            let metadata = entry.metadata().ok();
            files.push(WhiteboardFile {
                identifier: stem,
                whiteboard_identifier: whiteboard_identifier.clone(),
                mime_type,
                data,
                created_at: system_time_to_rfc3339(
                    metadata.as_ref().and_then(|m| m.created().ok()),
                ),
                updated_at: system_time_to_rfc3339(
                    metadata.as_ref().and_then(|m| m.modified().ok()),
                ),
            });
        }
    }

    // CDN fallback: check DB for files with remote_url not found locally
    let db_files = whiteboard_files::Entity::find()
        .filter(whiteboard_files::Column::WhiteboardIdentifier.eq(whiteboard_id))
        .all(state.db.as_ref())
        .await
        .unwrap_or_default();

    for db_file in db_files {
        let id_str = db_file.identifier.to_string();
        if local_ids.contains(&id_str) {
            continue;
        }

        let remote_url = match &db_file.remote_url {
            Some(url) if !url.is_empty() => url,
            _ => continue,
        };

        let response = match reqwest::get(remote_url.as_str()).await {
            Ok(r) if r.status().is_success() => r,
            Ok(r) => {
                log::warn!("CDN download failed for {remote_url}: {}", r.status());
                continue;
            }
            Err(e) => {
                log::warn!("CDN download failed for {remote_url}: {e}");
                continue;
            }
        };

        let bytes = match response.bytes().await {
            Ok(b) => b,
            Err(_) => continue,
        };

        let mime = &db_file.mime_type;
        let ext = extension_from_mime(mime);
        let data_url = bytes_to_data_url(&bytes, mime);

        // Re-cache locally as binary
        if let Ok(local_dir) = whiteboard_files_dir(&app, &whiteboard_id) {
            let _ = std::fs::create_dir_all(&local_dir);
            let local_path = local_dir.join(format!("{id_str}.{ext}"));
            let _ = std::fs::write(&local_path, &bytes);
        }

        files.push(WhiteboardFile {
            identifier: id_str,
            whiteboard_identifier: whiteboard_identifier.clone(),
            mime_type: mime.clone(),
            data: data_url,
            created_at: db_file.created_at.to_rfc3339(),
            updated_at: db_file.updated_at.to_rfc3339(),
        });
    }

    Ok(files)
}

#[tauri::command]
pub async fn delete_whiteboard_file(
    app: AppHandle,
    state: State<'_, AppState>,
    file_id: String,
    _meta: Option<RequestMeta>,
) -> Result<(), String> {
    let file_uuid = Uuid::parse_str(&file_id)
        .map_err(|_| "invalid file identifier".to_string())?;

    let root = whiteboards_root(&app)?;
    if root.exists() {
        for entry in std::fs::read_dir(&root)
            .map_err(|e| e.to_string())?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
        {
            if let Some(path) = find_file_by_uuid(&entry.path(), &file_uuid) {
                std::fs::remove_file(&path).map_err(|e| e.to_string())?;
                break;
            }
        }
    }

    let _ = whiteboard_files::Entity::delete_by_id(file_uuid)
        .exec(state.db.as_ref())
        .await;

    Ok(())
}

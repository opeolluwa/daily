use lunar::{
    adapters::meta::RequestMeta,
    entities::notes,
    repositories::notes::NotesRepositoryExt,
    repositories::workspace_manager::{DuplicateRecord, TransferRecord},
};
use tauri::{AppHandle, Manager, State};
use uuid::Uuid;

use crate::{
    adapters::notes::{CreateNote, UpdateNote},
    errors::AppError,
    state::app::AppState,
};

#[tauri::command]
pub async fn create_note(
    state: State<'_, AppState>,
    note: CreateNote,
    meta: Option<RequestMeta>,
) -> Result<notes::Model, AppError> {
    let created = state.notes_repository.create(&note.into(), &meta).await?;
    Ok(created)
}

#[tauri::command]
pub async fn get_note(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<Option<notes::Model>, AppError> {
    state
        .notes_repository
        .find_by_id(&identifier, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_all_notes(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<notes::Model>, AppError> {
    state
        .notes_repository
        .find_all(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn delete_note(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state.notes_repository.delete(&identifier, &meta).await?;
    Ok(())
}

#[tauri::command]
pub async fn update_note(
    state: State<'_, AppState>,
    identifier: Uuid,
    note: UpdateNote,
    meta: Option<RequestMeta>,
) -> Result<notes::Model, AppError> {
    let updated = state
        .notes_repository
        .update(&identifier, &note.into(), &meta)
        .await?;
    Ok(updated)
}

#[tauri::command]
pub async fn get_recently_added_notes(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<notes::Model>, AppError> {
    state
        .notes_repository
        .recently_added(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn duplicate_note(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    let new_identifier = state
        .notes_repository
        .duplicate_record(
            &record_identifier,
            &previous_workspace_identifier,
            &target_workspace_identifier,
        )
        .await
        .map_err(AppError::from)?;
    let meta = RequestMeta {
        workspace_identifier: target_workspace_identifier,
    };
    let _ = state
        .notes_repository
        .find_by_id(&new_identifier, &Some(meta))
        .await
        .map_err(AppError::from)?;
    Ok(())
}

#[tauri::command]
pub async fn transfer_note(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .notes_repository
        .transfer_record(
            &record_identifier,
            &previous_workspace_identifier,
            &target_workspace_identifier,
        )
        .await
        .map_err(AppError::from)?;
    Ok(())
}

#[derive(serde::Serialize)]
#[serde(tag = "status", rename_all = "camelCase")]
pub enum PdfExportStatus {
    Exported { path: String },
    Duplicate { path: String },
}

fn sanitize_file_name(title: &str) -> String {
    let sanitized: String = title
        .chars()
        .filter(|c| !matches!(c, '/' | '\\' | '\0'))
        .collect();
    let sanitized = sanitized.trim();

    if sanitized.is_empty() {
        "untitled".to_string()
    } else {
        sanitized.to_string()
    }
}

#[tauri::command]
pub async fn export_notes_as_pdf(
    app: AppHandle,
    state: State<'_, AppState>,
    record_identifier: Uuid,
    _previous_workspace_identifier: Uuid,
    meta: Option<RequestMeta>,
    overwrite: Option<bool>,
) -> Result<PdfExportStatus, AppError> {
    let note = state
        .notes_repository
        .find_by_id(&record_identifier, &meta)
        .await
        .map_err(|err| AppError::Io(err.to_string()))?;

    let note = note.ok_or_else(|| AppError::Io("note not found".to_string()))?;

    let download_dir = app
        .path()
        .download_dir()
        .map_err(|err| AppError::Io(err.to_string()))?;
    let file_name = sanitize_file_name(&note.title);
    let overwrite = overwrite.unwrap_or(false);

    let target = download_dir.join(format!("{}.pdf", file_name));
    if let Some(existing) = lunar::utils::detect_duplicate(&target)
        .await
        .map_err(|err| AppError::Io(err.to_string()))?
    {
        if !overwrite {
            return Ok(PdfExportStatus::Duplicate {
                path: existing.display().to_string(),
            });
        }
    }

    let path =
        lunar::ironpress::render_html_to_pdf(&note.content, &file_name, &download_dir, overwrite)
            .await
            .map_err(|err| AppError::Io(err.to_string()))?;

    Ok(PdfExportStatus::Exported {
        path: path.display().to_string(),
    })
}

use lunar::{
    adapters::meta::RequestMeta,
    entities::paste_bins,
    repositories::paste_bins::PasteBinRepositoryExt,
    repositories::workspace_manager::{DuplicateRecord, TransferRecord},
};
use tauri::State;
use uuid::Uuid;

use crate::{
    adapters::paste_bins::{CreatePasteBin, UpdatePasteBin},
    errors::AppError,
    state::app::AppState,
};

#[tauri::command]
pub async fn create_paste_bin(
    state: State<'_, AppState>,
    paste_bin: CreatePasteBin,
    meta: Option<RequestMeta>,
) -> Result<paste_bins::Model, AppError> {
    let created = state
        .paste_bin_repository
        .create(&paste_bin.into(), &meta)
        .await?;
    Ok(created)
}

#[tauri::command]
pub async fn get_paste_bin(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<Option<paste_bins::Model>, AppError> {
    state
        .paste_bin_repository
        .find_by_id(&identifier, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_all_paste_bins(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<paste_bins::Model>, AppError> {
    state
        .paste_bin_repository
        .find_all(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn delete_paste_bin(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .paste_bin_repository
        .delete(&identifier, &meta)
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn update_paste_bin(
    state: State<'_, AppState>,
    identifier: Uuid,
    paste_bin: UpdatePasteBin,
    meta: Option<RequestMeta>,
) -> Result<paste_bins::Model, AppError> {
    let updated = state
        .paste_bin_repository
        .update(&identifier, &paste_bin.into(), &meta)
        .await?;
    Ok(updated)
}

#[tauri::command]
pub async fn get_recently_added_paste_bins(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<paste_bins::Model>, AppError> {
    let paste_bins = state.paste_bin_repository.recently_added(&meta).await?;
    Ok(paste_bins)
}

#[tauri::command]
pub async fn duplicate_paste_bin(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    let new_identifier = state
        .paste_bin_repository
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
        .paste_bin_repository
        .find_by_id(&new_identifier, &Some(meta))
        .await
        .map_err(AppError::from)?;
    Ok(())
}

#[tauri::command]
pub async fn transfer_paste_bin(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .paste_bin_repository
        .transfer_record(
            &record_identifier,
            &previous_workspace_identifier,
            &target_workspace_identifier,
        )
        .await
        .map_err(AppError::from)?;
    Ok(())
}

use kernel::{
    adapters::backup_settings::UpdateBackupSettings, entities::backup_settings,
    repositories::backup_settings::BackupSettingsRepositoryExt,
};
use tauri::State;

use crate::{errors::AppError, state::app::AppState};

#[tauri::command]
pub async fn get_backup_settings(
    state: State<'_, AppState>,
) -> Result<backup_settings::Model, AppError> {
    state
        .backup_settings_repository
        .get_or_create()
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn update_backup_settings(
    state: State<'_, AppState>,
    identifier: String,
    settings: UpdateBackupSettings,
) -> Result<backup_settings::Model, AppError> {
    state
        .backup_settings_repository
        .update(&identifier, &settings)
        .await
        .map_err(Into::into)
}

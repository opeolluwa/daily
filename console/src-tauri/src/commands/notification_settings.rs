use kernel::{
    adapters::notification_settings::UpdateNotificationSettings,
    entities::notification_settings,
    repositories::notification_settings::NotificationSettingsRepositoryExt,
};
use tauri::State;

use crate::{errors::AppError, state::app::AppState};

#[tauri::command]
pub async fn get_notification_settings(
    state: State<'_, AppState>,
) -> Result<notification_settings::Model, AppError> {
    state
        .notification_settings_repository
        .get_or_create()
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn update_notification_settings(
    state: State<'_, AppState>,
    identifier: String,
    settings: UpdateNotificationSettings,
) -> Result<notification_settings::Model, AppError> {
    state
        .notification_settings_repository
        .update(&identifier, &settings)
        .await
        .map_err(Into::into)
}

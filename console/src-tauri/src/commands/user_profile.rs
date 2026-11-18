use kernel::{
    adapters::user_profile::UpdateUserProfile,
    entities::user_profile,
    repositories::user_profile::UserProfileRepositoryExt,
};
use tauri::State;

use crate::{errors::AppError, state::app::AppState};

#[tauri::command]
pub async fn get_user_profile(
    state: State<'_, AppState>,
) -> Result<user_profile::Model, AppError> {
    state
        .user_profile_repository
        .get_or_create()
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn update_user_profile(
    state: State<'_, AppState>,
    identifier: String,
    profile: UpdateUserProfile,
) -> Result<user_profile::Model, AppError> {
    state
        .user_profile_repository
        .update(&identifier, &profile)
        .await
        .map_err(Into::into)
}

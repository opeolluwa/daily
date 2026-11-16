use kernel::{
    adapters::{
        meta::RequestMeta,
        workspace::{CreateWorkspace, UpdateWorkspace},
    },
    repositories::workspace::WorkspaceRepositoryExt,
};
use tauri::State;

use crate::{errors::AppError, state::app::AppState};

#[tauri::command]
pub async fn create_workspace(
    state: State<'_, AppState>,
    workspace: CreateWorkspace,
) -> Result<kernel::entities::workspaces::Model, AppError> {
    let workspace = state
        .workspace_repository
        .create_workspace(workspace)
        .await?;
    Ok(workspace)
}

#[tauri::command]
pub async fn list_workspaces(
    state: State<'_, AppState>,
) -> Result<Vec<kernel::entities::workspaces::Model>, AppError> {
    let workspaces = state.workspace_repository.list_workspaces().await?;
    Ok(workspaces)
}

#[tauri::command]
pub async fn get_workspace_by_id(
    state: State<'_, AppState>,
    id: String,
) -> Result<kernel::entities::workspaces::Model, AppError> {
    let workspace = state.workspace_repository.get_workspace_by_id(&id).await?;
    Ok(workspace)
}

#[tauri::command]
pub async fn update_workspace(
    state: State<'_, AppState>,
    identifier: String,
    workspace: UpdateWorkspace,
) -> Result<kernel::entities::workspaces::Model, AppError> {
    let updated = state
        .workspace_repository
        .update_workspace(&identifier, workspace)
        .await?;
    Ok(updated)
}

#[tauri::command]
pub async fn verify_workspace_password(
    state: State<'_, AppState>,
    identifier: String,
    password: String,
) -> Result<bool, AppError> {
    let ok = state
        .workspace_repository
        .verify_workspace_password(&identifier, &password)
        .await?;
    Ok(ok)
}

#[tauri::command]
pub async fn delete_workspace(
    state: State<'_, AppState>,
    identifier: String,
    meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .workspace_repository
        .delete_workspace(&identifier, &meta)
        .await?;

    Ok(())
}

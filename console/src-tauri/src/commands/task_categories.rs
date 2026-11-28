use kernel::{
    adapters::meta::RequestMeta, entities::task_category,
    repositories::task_categories::TaskCategoryRepositoryExt,
};
use tauri::State;

use crate::{
    adapters::task_category::{CreateTaskCategory, UpdateTaskCategory},
    errors::AppError,
    state::app::AppState,
};

#[tauri::command]
pub async fn create_task_category(
    state: State<'_, AppState>,
    category: CreateTaskCategory,
    meta: Option<RequestMeta>,
) -> Result<task_category::Model, AppError> {
    let model = state
        .task_category_repository
        .create(&category.into(), &meta)
        .await
        .map_err(AppError::from)?;
    Ok(model)
}

#[tauri::command]
pub async fn get_all_task_categories(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<task_category::Model>, AppError> {
    state
        .task_category_repository
        .find_all(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn update_task_category(
    state: State<'_, AppState>,
    identifier: String,
    category: UpdateTaskCategory,
    meta: Option<RequestMeta>,
) -> Result<task_category::Model, AppError> {
    let model = state
        .task_category_repository
        .update(&identifier, &category.into(), &meta)
        .await
        .map_err(AppError::from)?;
    Ok(model)
}

#[tauri::command]
pub async fn delete_task_category(
    state: State<'_, AppState>,
    identifier: String,
    meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .task_category_repository
        .delete(&identifier, &meta)
        .await
        .map_err(AppError::from)?;
    Ok(())
}

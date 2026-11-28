use kernel::{
    adapters::{
        meta::RequestMeta,
        task_subtask::{CreateSubtask, SubtaskInput, TaskWithSubtasks, UpdateSubtask},
    },
    entities::task_subtask::Model as SubtaskModel,
    repositories::prelude::*,
};
use tauri::State;

use crate::{errors::AppError, state::app::AppState};

#[tauri::command]
pub async fn get_task_subtasks(
    state: State<'_, AppState>,
    task_identifier: String,
    meta: Option<RequestMeta>,
) -> Result<Vec<SubtaskModel>, AppError> {
    state
        .task_subtask_repository
        .find_by_task(&task_identifier, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_tasks_with_subtasks(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<TaskWithSubtasks>, AppError> {
    state
        .task_subtask_repository
        .find_all_with_subtasks(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn create_task_subtask(
    state: State<'_, AppState>,
    task_identifier: String,
    subtask: CreateSubtask,
    meta: Option<RequestMeta>,
) -> Result<SubtaskModel, AppError> {
    state
        .task_subtask_repository
        .create(&task_identifier, &subtask, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn update_task_subtask(
    state: State<'_, AppState>,
    identifier: String,
    subtask: UpdateSubtask,
    meta: Option<RequestMeta>,
) -> Result<SubtaskModel, AppError> {
    state
        .task_subtask_repository
        .update(&identifier, &subtask, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn delete_task_subtask(
    state: State<'_, AppState>,
    identifier: String,
    meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .task_subtask_repository
        .delete(&identifier, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn save_task_subtasks(
    state: State<'_, AppState>,
    task_identifier: String,
    subtasks: Vec<SubtaskInput>,
    meta: Option<RequestMeta>,
) -> Result<Vec<SubtaskModel>, AppError> {
    state
        .task_subtask_repository
        .save_all(&task_identifier, &subtasks, &meta)
        .await
        .map_err(Into::into)
}

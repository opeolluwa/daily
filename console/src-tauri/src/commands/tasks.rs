use kernel::{
    adapters::meta::RequestMeta,
    entities::sea_orm_active_enums::Priority as TaskPriority,
    entities::task,
    repositories::tasks::TaskRepositoryExt,
    repositories::workspace_manager::{DuplicateRecord, TransferRecord},
};
use tauri::State;
use uuid::Uuid;

use crate::{
    adapters::tasks::{CreateTask, UpdateTask},
    errors::AppError,
    state::app::AppState,
};

#[tauri::command]
pub async fn create_task(
    state: State<'_, AppState>,
    task: CreateTask,
    meta: Option<RequestMeta>,
) -> Result<task::Model, AppError> {
    let model = state
        .task_repository
        .create_task(&task.into(), &meta)
        .await
        .map_err(AppError::from)?;
    Ok(model)
}

#[tauri::command]
pub async fn get_task(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<Option<task::Model>, AppError> {
    state
        .task_repository
        .find_by_id(&identifier, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_all_tasks(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<task::Model>, AppError> {
    state
        .task_repository
        .find_all(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn update_task(
    state: State<'_, AppState>,
    identifier: Uuid,
    task: UpdateTask,
    meta: Option<RequestMeta>,
) -> Result<task::Model, AppError> {
    let model = state
        .task_repository
        .update(&identifier, &task.into(), &meta)
        .await
        .map_err(AppError::from)?;
    Ok(model)
}

#[tauri::command]
pub async fn delete_task(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .task_repository
        .delete(&identifier, &meta)
        .await
        .map_err(AppError::from)?;
    Ok(())
}

#[tauri::command]
pub async fn mark_task_done(
    state: State<'_, AppState>,
    identifier: Uuid,
    done: bool,
    meta: Option<RequestMeta>,
) -> Result<task::Model, AppError> {
    let model = state
        .task_repository
        .mark_done(&identifier, done, &meta)
        .await
        .map_err(AppError::from)?;
    Ok(model)
}

#[tauri::command]
pub async fn change_task_priority(
    state: State<'_, AppState>,
    identifier: Uuid,
    priority: String,
    meta: Option<RequestMeta>,
) -> Result<task::Model, AppError> {
    let priority = match priority.as_str() {
        "high" => TaskPriority::High,
        "low" => TaskPriority::Low,
        _ => TaskPriority::Medium,
    };
    let model = state
        .task_repository
        .change_priority(&identifier, &priority, &meta)
        .await
        .map_err(AppError::from)?;
    Ok(model)
}

#[tauri::command]
pub async fn update_task_due_date(
    state: State<'_, AppState>,
    identifier: Uuid,
    due_date: Option<String>,
    meta: Option<RequestMeta>,
) -> Result<task::Model, AppError> {
    use chrono::NaiveDate;

    let date = due_date
        .as_deref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

    let model = state
        .task_repository
        .update_due_date(&identifier, date, &meta)
        .await
        .map_err(AppError::from)?;
    Ok(model)
}

#[tauri::command]
pub async fn transfer_task(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .task_repository
        .transfer_record(
            &record_identifier,
            &previous_workspace_identifier,
            &target_workspace_identifier,
        )
        .await
        .map_err(AppError::from)?;
    Ok(())
}

#[tauri::command]
pub async fn duplicate_task(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    let new_identifier = state
        .task_repository
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
        .task_repository
        .find_by_id(&new_identifier, &Some(meta))
        .await
        .map_err(AppError::from)?;
    Ok(())
}

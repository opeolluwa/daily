use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, QueryOrder,
};
use wasm_bindgen::prelude::*;

use crate::adapters::meta::RequestMeta;
use crate::adapters::task_subtask::{CreateSubtask, SubtaskInput, TaskWithSubtasks, UpdateSubtask};
use crate::entities::sea_orm_active_enums::TaskStatus;
use crate::entities::{task, task_subtask};
use crate::error::KernelError;
use crate::utils::{extract_req_meta, js_err, mock_connection, to_js};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct TaskSubtaskRepository {
    conn: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait TaskSubtaskRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn find_by_task(
        &self,
        task_identifier: &str,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<task_subtask::Model>, KernelError>;

    async fn find_all_with_subtasks(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<TaskWithSubtasks>, KernelError>;

    async fn create(
        &self,
        task_identifier: &str,
        payload: &CreateSubtask,
        meta: &Option<RequestMeta>,
    ) -> Result<task_subtask::Model, KernelError>;

    async fn update(
        &self,
        identifier: &str,
        payload: &UpdateSubtask,
        meta: &Option<RequestMeta>,
    ) -> Result<task_subtask::Model, KernelError>;

    async fn delete(&self, identifier: &str, meta: &Option<RequestMeta>) -> Result<(), KernelError>;

    async fn save_all(
        &self,
        task_identifier: &str,
        subtasks: &[SubtaskInput],
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<task_subtask::Model>, KernelError>;
}

impl TaskSubtaskRepository {
    async fn ensure_parent_task(
        &self,
        task_identifier: &str,
        workspace_identifier: &str,
    ) -> Result<task::Model, KernelError> {
        task::Entity::find()
            .filter(task::Column::Identifier.eq(task_identifier))
            .filter(task::Column::WorkspaceIdentifier.eq(workspace_identifier))
            .filter(task::Column::DeletedAt.is_null())
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::DbOperationError("task not found".to_string()))
    }

    async fn find_models(
        &self,
        task_identifier: &str,
        workspace_identifier: &str,
    ) -> Result<Vec<task_subtask::Model>, KernelError> {
        task_subtask::Entity::find()
            .filter(task_subtask::Column::TaskIdentifier.eq(task_identifier))
            .filter(task_subtask::Column::WorkspaceIdentifier.eq(workspace_identifier))
            .filter(task_subtask::Column::DeletedAt.is_null())
            .order_by_asc(task_subtask::Column::SortOrder)
            .order_by_asc(task_subtask::Column::CreatedAt)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn sync_parent_completion(
        &self,
        task_identifier: &str,
        workspace_identifier: &str,
    ) -> Result<(), KernelError> {
        let subtasks = self
            .find_models(task_identifier, workspace_identifier)
            .await?;
        if subtasks.is_empty() {
            return Ok(());
        }

        let parent = self
            .ensure_parent_task(task_identifier, workspace_identifier)
            .await?;
        let completed = subtasks.iter().filter(|subtask| subtask.done).count();
        let all_done = completed == subtasks.len();
        let should_reopen = !all_done && (parent.done || parent.status == TaskStatus::Done);

        if !all_done && !should_reopen {
            return Ok(());
        }

        let mut active_model = parent.into_active_model();
        if all_done {
            if active_model.done.unwrap() && active_model.status.unwrap() == TaskStatus::Done {
                return Ok(());
            }
            active_model.done = Set(true);
            active_model.status = Set(TaskStatus::Done);
        } else {
            active_model.done = Set(false);
            active_model.status = Set(if completed == 0 {
                TaskStatus::Planned
            } else {
                TaskStatus::InProgress
            });
        }
        active_model.updated_at = Set(Utc::now().fixed_offset());
        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl TaskSubtaskRepositoryExt for TaskSubtaskRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    async fn find_by_task(
        &self,
        task_identifier: &str,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<task_subtask::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;
        self.find_models(task_identifier, &meta.workspace_identifier).await
    }

    async fn find_all_with_subtasks(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<TaskWithSubtasks>, KernelError> {
        let meta = extract_req_meta(meta)?;

        let tasks = task::Entity::find()
            .filter(task::Column::WorkspaceIdentifier.eq(meta.workspace_identifier.clone()))
            .filter(task::Column::DeletedAt.is_null())
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        let subtasks = task_subtask::Entity::find()
            .filter(task_subtask::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .filter(task_subtask::Column::DeletedAt.is_null())
            .order_by_asc(task_subtask::Column::SortOrder)
            .order_by_asc(task_subtask::Column::CreatedAt)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        let mut grouped: HashMap<String, Vec<task_subtask::Model>> = HashMap::new();
        for subtask in subtasks {
            grouped
                .entry(subtask.task_identifier.clone())
                .or_default()
                .push(subtask);
        }

        Ok(tasks
            .into_iter()
            .map(|task| {
                let subtasks = grouped.remove(&task.identifier).unwrap_or_default();
                let completed = subtasks.iter().filter(|subtask| subtask.done).count();
                let progress = if subtasks.is_empty() {
                    0
                } else {
                    ((completed as f64 / subtasks.len() as f64) * 100.0).round() as u8
                };
                TaskWithSubtasks {
                    task,
                    subtasks,
                    progress,
                }
            })
            .collect())
    }

    async fn create(
        &self,
        task_identifier: &str,
        payload: &CreateSubtask,
        meta: &Option<RequestMeta>,
    ) -> Result<task_subtask::Model, KernelError> {
        let meta = extract_req_meta(meta)?;
        self.ensure_parent_task(task_identifier, &meta.workspace_identifier)
            .await?;

        let mut active_model: task_subtask::ActiveModel = payload.to_owned().into();
        active_model.task_identifier = Set(task_identifier.to_string());
        active_model.workspace_identifier = Set(Some(meta.workspace_identifier.clone()));
        let model = active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        self.sync_parent_completion(task_identifier, &meta.workspace_identifier)
            .await?;
        Ok(model)
    }

    async fn update(
        &self,
        identifier: &str,
        payload: &UpdateSubtask,
        meta: &Option<RequestMeta>,
    ) -> Result<task_subtask::Model, KernelError> {
        let meta = extract_req_meta(meta)?;
        let model = task_subtask::Entity::find()
            .filter(task_subtask::Column::Identifier.eq(identifier))
            .filter(task_subtask::Column::WorkspaceIdentifier.eq(meta.workspace_identifier.clone()))
            .filter(task_subtask::Column::DeletedAt.is_null())
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::DbOperationError("subtask not found".to_string()))?;

        let task_identifier = model.task_identifier.clone();
        let mut active_model = model.into_active_model();
        if let Some(title) = &payload.title {
            active_model.title = Set(title.clone());
        }
        if let Some(done) = payload.done {
            active_model.done = Set(done);
        }
        if let Some(sort_order) = payload.sort_order {
            active_model.sort_order = Set(sort_order);
        }
        active_model.updated_at = Set(Utc::now().fixed_offset());
        let model = active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        self.sync_parent_completion(&task_identifier, &meta.workspace_identifier)
            .await?;
        Ok(model)
    }

    async fn delete(&self, identifier: &str, meta: &Option<RequestMeta>) -> Result<(), KernelError> {
        let meta = extract_req_meta(meta)?;
        let model = task_subtask::Entity::find()
            .filter(task_subtask::Column::Identifier.eq(identifier))
            .filter(task_subtask::Column::WorkspaceIdentifier.eq(meta.workspace_identifier.clone()))
            .filter(task_subtask::Column::DeletedAt.is_null())
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::DbOperationError("subtask not found".to_string()))?;

        let task_identifier = model.task_identifier.clone();
        let mut active_model = model.into_active_model();
        active_model.deleted_at = Set(Some(Utc::now().fixed_offset()));
        active_model.updated_at = Set(Utc::now().fixed_offset());
        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        self.sync_parent_completion(&task_identifier, &meta.workspace_identifier)
            .await?;
        Ok(())
    }

    async fn save_all(
        &self,
        task_identifier: &str,
        subtasks: &[SubtaskInput],
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<task_subtask::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;
        self.ensure_parent_task(task_identifier, &meta.workspace_identifier)
            .await?;

        let existing = self
            .find_models(task_identifier, &meta.workspace_identifier)
            .await?;
        let mut existing_by_id: HashMap<String, task_subtask::Model> = existing
            .into_iter()
            .map(|model| (model.identifier.clone(), model))
            .collect();
        let mut kept_ids = Vec::new();

        for (index, payload) in subtasks.iter().enumerate() {
            let title = payload.title.trim();
            if title.is_empty() {
                continue;
            }

            if let Some(identifier) = &payload.identifier {
                if let Some(model) = existing_by_id.remove(identifier) {
                    let mut active_model = model.into_active_model();
                    active_model.title = Set(title.to_string());
                    active_model.done = Set(payload.done.unwrap_or(false));
                    active_model.sort_order = Set(payload.sort_order.unwrap_or(index as i32));
                    active_model.updated_at = Set(Utc::now().fixed_offset());
                    let saved = active_model
                        .update(self.conn.as_ref())
                        .await
                        .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
                    kept_ids.push(saved.identifier);
                    continue;
                }
            }

            let mut active_model: task_subtask::ActiveModel = CreateSubtask {
                title: title.to_string(),
                sort_order: Some(payload.sort_order.unwrap_or(index as i32)),
            }
            .into();
            active_model.task_identifier = Set(task_identifier.to_string());
            active_model.done = Set(payload.done.unwrap_or(false));
            active_model.workspace_identifier = Set(Some(meta.workspace_identifier.clone()));
            let saved = active_model
                .insert(self.conn.as_ref())
                .await
                .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
            kept_ids.push(saved.identifier);
        }

        for model in existing_by_id.into_values() {
            let mut active_model = model.into_active_model();
            active_model.deleted_at = Set(Some(Utc::now().fixed_offset()));
            active_model.updated_at = Set(Utc::now().fixed_offset());
            active_model
                .update(self.conn.as_ref())
                .await
                .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        }

        let models = self
            .find_models(task_identifier, &meta.workspace_identifier)
            .await?;
        self.sync_parent_completion(task_identifier, &meta.workspace_identifier)
            .await?;
        Ok(models)
    }
}

#[wasm_bindgen]
impl TaskSubtaskRepository {
    #[wasm_bindgen(constructor)]
    pub fn new_wasm() -> Self {
        Self::new(mock_connection())
    }

    #[wasm_bindgen(js_name = "find_by_task")]
    pub async fn find_by_task_js(
        &self,
        task_identifier: &str,
        meta: JsValue,
    ) -> Result<JsValue, JsValue> {
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let models = <Self as TaskSubtaskRepositoryExt>::find_by_task(
            self,
            task_identifier,
            &meta,
        )
        .await?;
        to_js(&models)
    }

    #[wasm_bindgen(js_name = "find_all_with_subtasks")]
    pub async fn find_all_with_subtasks_js(&self, meta: JsValue) -> Result<JsValue, JsValue> {
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let models =
            <Self as TaskSubtaskRepositoryExt>::find_all_with_subtasks(self, &meta).await?;
        to_js(&models)
    }

    #[wasm_bindgen(js_name = "create")]
    pub async fn create_js(
        &self,
        task_identifier: &str,
        payload: JsValue,
        meta: JsValue,
    ) -> Result<JsValue, JsValue> {
        let payload: CreateSubtask = serde_wasm_bindgen::from_value(payload).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let model =
            <Self as TaskSubtaskRepositoryExt>::create(self, task_identifier, &payload, &meta)
                .await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "update")]
    pub async fn update_js(
        &self,
        identifier: &str,
        payload: JsValue,
        meta: JsValue,
    ) -> Result<JsValue, JsValue> {
        let payload: UpdateSubtask = serde_wasm_bindgen::from_value(payload).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let model = <Self as TaskSubtaskRepositoryExt>::update(self, identifier, &payload, &meta)
            .await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "delete")]
    pub async fn delete_js(&self, identifier: &str, meta: JsValue) -> Result<JsValue, JsValue> {
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        <Self as TaskSubtaskRepositoryExt>::delete(self, identifier, &meta).await?;
        Ok(JsValue::UNDEFINED)
    }

    #[wasm_bindgen(js_name = "save_all")]
    pub async fn save_all_js(
        &self,
        task_identifier: &str,
        subtasks: JsValue,
        meta: JsValue,
    ) -> Result<JsValue, JsValue> {
        let subtasks: Vec<SubtaskInput> =
            serde_wasm_bindgen::from_value(subtasks).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let models = <Self as TaskSubtaskRepositoryExt>::save_all(
            self,
            task_identifier,
            &subtasks,
            &meta,
        )
        .await?;
        to_js(&models)
    }
}

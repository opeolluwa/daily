use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use wasm_bindgen::prelude::*;

use crate::adapters::meta::RequestMeta;
use crate::adapters::task_category::{CreateTaskCategory, UpdateTaskCategory};
use crate::entities::task_category;
use crate::error::KernelError;
use crate::utils::{extract_req_meta, js_err, mock_connection, to_js};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct TaskCategoryRepository {
    conn: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait TaskCategoryRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn create(
        &self,
        payload: &CreateTaskCategory,
        meta: &Option<RequestMeta>,
    ) -> Result<task_category::Model, KernelError>;

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<task_category::Model>, KernelError>;

    async fn find_by_id(
        &self,
        identifier: &str,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<task_category::Model>, KernelError>;

    async fn update(
        &self,
        identifier: &str,
        payload: &UpdateTaskCategory,
        meta: &Option<RequestMeta>,
    ) -> Result<task_category::Model, KernelError>;

    async fn delete(
        &self,
        identifier: &str,
        meta: &Option<RequestMeta>,
    ) -> Result<(), KernelError>;
}

#[async_trait]
impl TaskCategoryRepositoryExt for TaskCategoryRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    async fn create(
        &self,
        payload: &CreateTaskCategory,
        meta: &Option<RequestMeta>,
    ) -> Result<task_category::Model, KernelError> {
        let mut active_model: task_category::ActiveModel = payload.to_owned().into();

        if let Some(meta) = meta {
            active_model.workspace_identifier = Set(Some(meta.workspace_identifier.clone()));
        } else {
            return Err(KernelError::DbOperationError(
                "workspace identifier is required".into(),
            ));
        };

        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<task_category::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        task_category::Entity::find()
            .filter(
                task_category::Column::WorkspaceIdentifier.eq(meta.workspace_identifier.clone()),
            )
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_by_id(
        &self,
        identifier: &str,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<task_category::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        task_category::Entity::find()
            .filter(task_category::Column::Identifier.eq(identifier))
            .filter(
                task_category::Column::WorkspaceIdentifier.eq(meta.workspace_identifier.clone()),
            )
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn update(
        &self,
        identifier: &str,
        payload: &UpdateTaskCategory,
        meta: &Option<RequestMeta>,
    ) -> Result<task_category::Model, KernelError> {
        let meta = extract_req_meta(meta)?;

        let model = task_category::Entity::find()
            .filter(task_category::Column::Identifier.eq(identifier))
            .filter(
                task_category::Column::WorkspaceIdentifier.eq(meta.workspace_identifier.clone()),
            )
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| {
                KernelError::DbOperationError("task category not found".to_string())
            })?;

        let mut active_model = model.into_active_model();

        if let Some(name) = &payload.name {
            active_model.name = Set(name.clone());
        }
        if let Some(color) = &payload.color {
            active_model.color = Set(color.clone());
        }
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn delete(
        &self,
        identifier: &str,
        meta: &Option<RequestMeta>,
    ) -> Result<(), KernelError> {
        let meta = extract_req_meta(meta)?;

        task_category::Entity::delete_many()
            .filter(task_category::Column::Identifier.eq(identifier))
            .filter(
                task_category::Column::WorkspaceIdentifier.eq(meta.workspace_identifier.clone()),
            )
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        Ok(())
    }
}

#[wasm_bindgen]
impl TaskCategoryRepository {
    #[wasm_bindgen(constructor)]
    pub fn new_wasm() -> Self {
        Self::new(mock_connection())
    }

    #[wasm_bindgen(js_name = "create")]
    pub async fn create_js(
        &self,
        payload: JsValue,
        meta: JsValue,
    ) -> Result<JsValue, JsValue> {
        let payload: CreateTaskCategory =
            serde_wasm_bindgen::from_value(payload).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let model =
            <Self as TaskCategoryRepositoryExt>::create(self, &payload, &meta).await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "find_all")]
    pub async fn find_all_js(&self, meta: JsValue) -> Result<JsValue, JsValue> {
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let models = <Self as TaskCategoryRepositoryExt>::find_all(self, &meta).await?;
        to_js(&models)
    }

    #[wasm_bindgen(js_name = "find_by_id")]
    pub async fn find_by_id_js(
        &self,
        identifier: &str,
        meta: JsValue,
    ) -> Result<JsValue, JsValue> {
        let id = identifier.to_string();
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let model =
            <Self as TaskCategoryRepositoryExt>::find_by_id(self, &id, &meta).await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "update")]
    pub async fn update_js(
        &self,
        identifier: &str,
        payload: JsValue,
        meta: JsValue,
    ) -> Result<JsValue, JsValue> {
        let id = identifier.to_string();
        let payload: UpdateTaskCategory =
            serde_wasm_bindgen::from_value(payload).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let model =
            <Self as TaskCategoryRepositoryExt>::update(self, &id, &payload, &meta).await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "delete")]
    pub async fn delete_js(
        &self,
        identifier: &str,
        meta: JsValue,
    ) -> Result<JsValue, JsValue> {
        let id = identifier.to_string();
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        <Self as TaskCategoryRepositoryExt>::delete(self, &id, &meta).await?;
        Ok(JsValue::UNDEFINED)
    }
}

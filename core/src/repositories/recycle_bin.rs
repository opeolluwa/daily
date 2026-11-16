use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection,
    EntityTrait, IntoActiveModel, QueryFilter, QueryOrder,
};
use wasm_bindgen::prelude::*;

use crate::entities::sea_orm_active_enums::ItemType;
use crate::types::EntitySyncResult;
use crate::{
    adapters::{meta::RequestMeta, recycle_bin::CreateRecycleBinEntry},
    entities::{bookmark, notes, paste_bins, recycle_bin, reminder, snippets, task},
    error::KernelError,
    utils::{extract_req_meta, js_err, mock_connection, to_js},
};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct RecycleBinRepository {
    conn: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait RecycleBinRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn store(
        &self,
        payload: &CreateRecycleBinEntry,
        meta: &Option<RequestMeta>,
    ) -> Result<recycle_bin::Model, KernelError>;

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<recycle_bin::Model>, KernelError>;

    async fn find_by_id(
        &self,
        identifier: &str,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<recycle_bin::Model>, KernelError>;

    async fn find_by_item_type(
        &self,
        item_type: &ItemType,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<recycle_bin::Model>, KernelError>;

    async fn purge(&self, identifier: &str, meta: &Option<RequestMeta>)
    -> Result<(), KernelError>;

    async fn purge_all(&self, meta: &Option<RequestMeta>) -> Result<Vec<String>, KernelError>;

    async fn restore(
        &self,
        identifier: &str,
        meta: &Option<RequestMeta>,
    ) -> Result<recycle_bin::Model, KernelError>;

    async fn upsert_many(
        &self,
        models: Vec<recycle_bin::Model>,
    ) -> Result<Vec<EntitySyncResult>, KernelError>;
}

#[async_trait]
impl RecycleBinRepositoryExt for RecycleBinRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    async fn store(
        &self,
        payload: &CreateRecycleBinEntry,
        meta: &Option<RequestMeta>,
    ) -> Result<recycle_bin::Model, KernelError> {
        let mut active_model: recycle_bin::ActiveModel = payload.to_owned().into();

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
    ) -> Result<Vec<recycle_bin::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        recycle_bin::Entity::find()
            .filter(recycle_bin::Column::WorkspaceIdentifier.eq(meta.workspace_identifier.clone()))
            .order_by_desc(recycle_bin::Column::DeletedAt)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_by_id(
        &self,
        identifier: &str,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<recycle_bin::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        recycle_bin::Entity::find()
            .filter(recycle_bin::Column::Identifier.eq(identifier))
            .filter(recycle_bin::Column::WorkspaceIdentifier.eq(meta.workspace_identifier.clone()))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_by_item_type(
        &self,
        item_type: &ItemType,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<recycle_bin::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        recycle_bin::Entity::find()
            .filter(recycle_bin::Column::WorkspaceIdentifier.eq(meta.workspace_identifier.clone()))
            .filter(recycle_bin::Column::ItemType.eq(item_type.to_owned()))
            .order_by_desc(recycle_bin::Column::DeletedAt)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn purge(
        &self,
        identifier: &str,
        meta: &Option<RequestMeta>,
    ) -> Result<(), KernelError> {
        let meta = extract_req_meta(meta)?;

        recycle_bin::Entity::delete_many()
            .filter(recycle_bin::Column::Identifier.eq(identifier))
            .filter(recycle_bin::Column::WorkspaceIdentifier.eq(meta.workspace_identifier.clone()))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }

    async fn purge_all(&self, meta: &Option<RequestMeta>) -> Result<Vec<String>, KernelError> {
        let meta = extract_req_meta(meta)?;

        let entries = recycle_bin::Entity::find()
            .filter(recycle_bin::Column::WorkspaceIdentifier.eq(meta.workspace_identifier.clone()))
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        let identifiers = entries
            .into_iter()
            .map(|entry| entry.identifier)
            .collect::<Vec<String>>();

        // the found entity should be what is passed to delete
        recycle_bin::Entity::delete_many()
            .filter(recycle_bin::Column::WorkspaceIdentifier.eq(meta.workspace_identifier.clone()))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(identifiers)
    }

    async fn restore(
        &self,
        identifier: &str,
        meta: &Option<RequestMeta>,
    ) -> Result<recycle_bin::Model, KernelError> {
        let meta = extract_req_meta(meta)?;

        let entry = recycle_bin::Entity::find()
            .filter(recycle_bin::Column::Identifier.eq(identifier))
            .filter(recycle_bin::Column::WorkspaceIdentifier.eq(meta.workspace_identifier.clone()))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| {
                KernelError::DbOperationError("recycle bin entry not found".to_string())
            })?;

        macro_rules! restore_soft_deleted {
            ($entity:ident, $model:ty, $active:ty) => {{
                let model: $model = Self::parse_payload(&entry.payload)?;
                let result = $entity::Entity::update_many()
                    .col_expr($entity::Column::DeletedAt, Expr::cust("NULL"))
                    .col_expr($entity::Column::UpdatedAt, Expr::current_timestamp())
                    .filter($entity::Column::Identifier.eq(model.identifier.clone()))
                    .exec(self.conn.as_ref())
                    .await
                    .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

                if result.rows_affected == 0 {
                    let _ = Self::restore_payload::<$model, $active>(
                        &entry.payload,
                        self.conn.as_ref(),
                    )
                    .await?;
                }
            }};
        }

        match entry.item_type {
            ItemType::Note => {
                restore_soft_deleted!(notes, notes::Model, notes::ActiveModel);
            }
            ItemType::Task => {
                restore_soft_deleted!(task, task::Model, task::ActiveModel);
            }
            ItemType::Bookmark => {
                restore_soft_deleted!(bookmark, bookmark::Model, bookmark::ActiveModel);
            }
            ItemType::Snippet => {
                restore_soft_deleted!(snippets, snippets::Model, snippets::ActiveModel);
            }
            ItemType::Reminder => {
                restore_soft_deleted!(reminder, reminder::Model, reminder::ActiveModel);
            }
            ItemType::PasteBin => {
                restore_soft_deleted!(paste_bins, paste_bins::Model, paste_bins::ActiveModel);
            }
        }

        recycle_bin::Entity::delete_many()
            .filter(recycle_bin::Column::Identifier.eq(identifier))
            .filter(recycle_bin::Column::WorkspaceIdentifier.eq(meta.workspace_identifier.clone()))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(entry)
    }

    async fn upsert_many(
        &self,
        models: Vec<recycle_bin::Model>,
    ) -> Result<Vec<EntitySyncResult>, KernelError> {
        let mut sync_results: Vec<EntitySyncResult> = Vec::new();
        for chunk in models.chunks(20) {
            let futures: Vec<_> = chunk
                .iter()
                .map(|model| {
                    let conn = self.conn.clone();
                    let model = model.clone();
                    async move {
                        let identifier = model.identifier.to_string();
                        let op_result: Result<(), KernelError> = async {
                            let exists = recycle_bin::Entity::find()
                                .filter(recycle_bin::Column::Identifier.eq(model.identifier.clone()))
                                .one(conn.as_ref())
                                .await
                                .map_err(|err| KernelError::DbOperationError(err.to_string()))?
                                .is_some();

                            let active_model = model.into_active_model();

                            if exists {
                                active_model.update(conn.as_ref()).await.map_err(|err| {
                                    KernelError::DbOperationError(err.to_string())
                                })?;
                            } else {
                                active_model.insert(conn.as_ref()).await.map_err(|err| {
                                    KernelError::DbOperationError(err.to_string())
                                })?;
                            }
                            Ok(())
                        }
                        .await;
                        EntitySyncResult {
                            identifier,
                            success: op_result.is_ok(),
                            error_message: op_result.err().map(|e| e.to_string()),
                        }
                    }
                })
                .collect();

            let chunk_results = futures::future::join_all(futures).await;
            sync_results.extend(chunk_results);
        }
        Ok(sync_results)
    }
}

// TODO: consider using https://crates.io/crates/heck
fn camelize_key(key: &str) -> String {
    let mut out = String::with_capacity(key.len());
    let mut upper = false;
    for ch in key.chars() {
        if ch == '_' {
            upper = true;
        } else if upper {
            out.extend(ch.to_uppercase());
            upper = false;
        } else {
            out.push(ch);
        }
    }
    out
}

impl RecycleBinRepository {
    fn camelized_value(payload: &str) -> Result<serde_json::Value, KernelError> {
        let value: serde_json::Value = serde_json::from_str(payload)
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        let Some(object) = value.as_object() else {
            return Ok(value);
        };
        Ok(serde_json::Value::Object(
            object
                .iter()
                .map(|(key, value)| (camelize_key(key), value.clone()))
                .collect(),
        ))
    }

    fn parse_payload<T: serde::de::DeserializeOwned>(payload: &str) -> Result<T, KernelError> {
        serde_json::from_value(Self::camelized_value(payload)?)
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    fn restore_value(payload: &str) -> Result<serde_json::Value, KernelError> {
        let mut value = Self::camelized_value(payload)?;
        if let Some(object) = value.as_object_mut() {
            object.insert("deleted_at".to_string(), serde_json::Value::Null);
            object.insert(
                "updated_at".to_string(),
                serde_json::Value::String(Utc::now().to_rfc3339()),
            );
        }
        Ok(value)
    }

    async fn restore_payload<M, A>(
        payload: &str,
        conn: &DatabaseConnection,
    ) -> Result<(), KernelError>
    where
        M: IntoActiveModel<A> + serde::de::DeserializeOwned,
        A: ActiveModelBehavior + Send,
        <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
    {
        let model: M = serde_json::from_value(Self::restore_value(payload)?)
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        model
            .into_active_model()
            .insert(conn)
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }
}

#[wasm_bindgen]
impl RecycleBinRepository {
    #[wasm_bindgen(constructor)]
    pub fn new_wasm() -> Self {
        Self::new(mock_connection())
    }

    #[wasm_bindgen(js_name = "store")]
    pub async fn store_js(&self, payload: JsValue, meta: JsValue) -> Result<JsValue, JsValue> {
        let payload: CreateRecycleBinEntry =
            serde_wasm_bindgen::from_value(payload).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let model = <Self as RecycleBinRepositoryExt>::store(self, &payload, &meta).await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "find_all")]
    pub async fn find_all_js(&self, meta: JsValue) -> Result<JsValue, JsValue> {
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let models = <Self as RecycleBinRepositoryExt>::find_all(self, &meta).await?;
        to_js(&models)
    }

    #[wasm_bindgen(js_name = "find_by_id")]
    pub async fn find_by_id_js(&self, identifier: &str, meta: JsValue) -> Result<JsValue, JsValue> {
        let id = identifier.to_string();
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let model = <Self as RecycleBinRepositoryExt>::find_by_id(self, &id, &meta).await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "find_by_item_type")]
    pub async fn find_by_item_type_js(
        &self,
        item_type: JsValue,
        meta: JsValue,
    ) -> Result<JsValue, JsValue> {
        let item_type: ItemType = serde_wasm_bindgen::from_value(item_type).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let models =
            <Self as RecycleBinRepositoryExt>::find_by_item_type(self, &item_type, &meta).await?;
        to_js(&models)
    }

    #[wasm_bindgen(js_name = "purge")]
    pub async fn purge_js(&self, identifier: &str, meta: JsValue) -> Result<JsValue, JsValue> {
        let id = identifier.to_string();
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        <Self as RecycleBinRepositoryExt>::purge(self, &id, &meta).await?;
        Ok(JsValue::UNDEFINED)
    }

    #[wasm_bindgen(js_name = "purge_all")]
    pub async fn purge_all_js(&self, meta: JsValue) -> Result<JsValue, JsValue> {
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        <Self as RecycleBinRepositoryExt>::purge_all(self, &meta).await?;
        Ok(JsValue::UNDEFINED)
    }

    #[wasm_bindgen(js_name = "restore")]
    pub async fn restore_js(&self, identifier: &str, meta: JsValue) -> Result<JsValue, JsValue> {
        let id = identifier.to_string();
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        <Self as RecycleBinRepositoryExt>::restore(self, &id, &meta).await?;
        Ok(JsValue::UNDEFINED)
    }
}

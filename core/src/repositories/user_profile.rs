use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use wasm_bindgen::prelude::*;

use crate::types::EntitySyncResult;
use crate::{
    adapters::user_profile::UpdateUserProfile,
    entities::user_profile,
    error::KernelError,
    id::new_id,
    utils::{js_err, mock_connection, to_js},
};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct UserProfileRepository {
    conn: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait UserProfileRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn get_or_create(&self) -> Result<user_profile::Model, KernelError>;

    async fn update(
        &self,
        identifier: &str,
        payload: &UpdateUserProfile,
    ) -> Result<user_profile::Model, KernelError>;

    async fn upsert_many(
        &self,
        models: Vec<user_profile::Model>,
    ) -> Result<Vec<EntitySyncResult>, KernelError>;
}

#[async_trait]
impl UserProfileRepositoryExt for UserProfileRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    async fn get_or_create(&self) -> Result<user_profile::Model, KernelError> {
        let existing = user_profile::Entity::find()
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        if let Some(model) = existing {
            return Ok(model);
        }

        let now = Utc::now().fixed_offset();
        let active_model = user_profile::ActiveModel {
            identifier: Set(new_id()),
            first_name: Set(String::new()),
            last_name: Set(String::new()),
            email: Set(String::new()),
            profile_picture: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn update(
        &self,
        identifier: &str,
        payload: &UpdateUserProfile,
    ) -> Result<user_profile::Model, KernelError> {
        let model = user_profile::Entity::find_by_id(identifier)
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| {
                KernelError::DbOperationError("user profile not found".to_string())
            })?;

        let mut active_model = model.into_active_model();

        if let Some(first_name) = &payload.first_name {
            active_model.first_name = Set(first_name.clone());
        }
        if let Some(last_name) = &payload.last_name {
            active_model.last_name = Set(last_name.clone());
        }
        if let Some(email) = &payload.email {
            active_model.email = Set(email.clone());
        }
        if let Some(profile_picture) = &payload.profile_picture {
            active_model.profile_picture = Set(Some(profile_picture.clone()));
        }
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn upsert_many(
        &self,
        models: Vec<user_profile::Model>,
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
                            let exists = user_profile::Entity::find()
                                .filter(user_profile::Column::Identifier.eq(model.identifier.clone()))
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

#[wasm_bindgen]
impl UserProfileRepository {
    #[wasm_bindgen(constructor)]
    pub fn new_wasm() -> Self {
        Self::new(mock_connection())
    }

    #[wasm_bindgen(js_name = "get_or_create")]
    pub async fn get_or_create_js(&self) -> Result<JsValue, JsValue> {
        let model =
            <Self as UserProfileRepositoryExt>::get_or_create(self).await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "update")]
    pub async fn update_js(
        &self,
        identifier: &str,
        payload: JsValue,
    ) -> Result<JsValue, JsValue> {
        let payload: UpdateUserProfile =
            serde_wasm_bindgen::from_value(payload).map_err(js_err)?;
        let model =
            <Self as UserProfileRepositoryExt>::update(self, identifier, &payload)
                .await?;
        to_js(&model)
    }
}

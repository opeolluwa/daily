use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, IntoActiveModel,
};
use wasm_bindgen::prelude::*;

use crate::{
    adapters::backup_settings::UpdateBackupSettings,
    entities::{backup_settings, sea_orm_active_enums::BackupMode},
    error::KernelError,
    id::new_id,
    utils::{js_err, mock_connection, to_js},
};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct BackupSettingsRepository {
    conn: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait BackupSettingsRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn get_or_create(&self) -> Result<backup_settings::Model, KernelError>;

    async fn update(
        &self,
        identifier: &str,
        payload: &UpdateBackupSettings,
    ) -> Result<backup_settings::Model, KernelError>;
}

#[async_trait]
impl BackupSettingsRepositoryExt for BackupSettingsRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    async fn get_or_create(&self) -> Result<backup_settings::Model, KernelError> {
        let existing = backup_settings::Entity::find()
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        if let Some(model) = existing {
            return Ok(model);
        }

        let now = Utc::now().fixed_offset();
        let active_model = backup_settings::ActiveModel {
            identifier: Set(new_id()),
            user_identifier: Set(None),
            server_url: Set(String::new()),
            last_sync: Set(None),
            backup_mode: Set(BackupMode::LocalOnly),
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
        payload: &UpdateBackupSettings,
    ) -> Result<backup_settings::Model, KernelError> {
        let model = backup_settings::Entity::find_by_id(identifier)
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| {
                KernelError::BackupSettingsNotFound(identifier.to_string())
            })?;

        let mut active_model = model.into_active_model();

        if let Some(val) = &payload.server_url {
            active_model.server_url = Set(val.clone());
        }
        if let Some(val) = &payload.backup_mode {
            active_model.backup_mode = Set(val.clone());
        }
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }
}

#[wasm_bindgen]
impl BackupSettingsRepository {
    #[wasm_bindgen(constructor)]
    pub fn new_wasm() -> Self {
        Self::new(mock_connection())
    }

    #[wasm_bindgen(js_name = "get_or_create")]
    pub async fn get_or_create_js(&self) -> Result<JsValue, JsValue> {
        let model =
            <Self as BackupSettingsRepositoryExt>::get_or_create(self).await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "update")]
    pub async fn update_js(
        &self,
        identifier: &str,
        payload: JsValue,
    ) -> Result<JsValue, JsValue> {
        let payload: UpdateBackupSettings =
            serde_wasm_bindgen::from_value(payload).map_err(js_err)?;
        let model =
            <Self as BackupSettingsRepositoryExt>::update(self, identifier, &payload)
                .await?;
        to_js(&model)
    }
}

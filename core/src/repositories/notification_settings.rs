use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, IntoActiveModel,
};
use wasm_bindgen::prelude::*;

use crate::{
    adapters::notification_settings::UpdateNotificationSettings,
    entities::notification_settings,
    error::KernelError,
    id::new_id,
    utils::{js_err, mock_connection, to_js},
};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct NotificationSettingsRepository {
    conn: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait NotificationSettingsRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn get_or_create(&self) -> Result<notification_settings::Model, KernelError>;

    async fn update(
        &self,
        identifier: &str,
        payload: &UpdateNotificationSettings,
    ) -> Result<notification_settings::Model, KernelError>;
}

#[async_trait]
impl NotificationSettingsRepositoryExt for NotificationSettingsRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    async fn get_or_create(&self) -> Result<notification_settings::Model, KernelError> {
        let existing = notification_settings::Entity::find()
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        if let Some(model) = existing {
            return Ok(model);
        }

        let now = Utc::now().fixed_offset();
        let active_model = notification_settings::ActiveModel {
            identifier: Set(new_id()),
            tasks_due: Set(true),
            reminders: Set(true),
            sync_complete: Set(false),
            app_updates: Set(true),
            user_identifier: Set(None),
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
        payload: &UpdateNotificationSettings,
    ) -> Result<notification_settings::Model, KernelError> {
        let model = notification_settings::Entity::find_by_id(identifier)
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| {
                KernelError::NotificationSettingsNotFound(identifier.to_string())
            })?;

        let mut active_model = model.into_active_model();

        if let Some(val) = payload.tasks_due {
            active_model.tasks_due = Set(val);
        }
        if let Some(val) = payload.reminders {
            active_model.reminders = Set(val);
        }
        if let Some(val) = payload.sync_complete {
            active_model.sync_complete = Set(val);
        }
        if let Some(val) = payload.app_updates {
            active_model.app_updates = Set(val);
        }
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }
}

#[wasm_bindgen]
impl NotificationSettingsRepository {
    #[wasm_bindgen(constructor)]
    pub fn new_wasm() -> Self {
        Self::new(mock_connection())
    }

    #[wasm_bindgen(js_name = "get_or_create")]
    pub async fn get_or_create_js(&self) -> Result<JsValue, JsValue> {
        let model =
            <Self as NotificationSettingsRepositoryExt>::get_or_create(self).await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "update")]
    pub async fn update_js(
        &self,
        identifier: &str,
        payload: JsValue,
    ) -> Result<JsValue, JsValue> {
        let payload: UpdateNotificationSettings =
            serde_wasm_bindgen::from_value(payload).map_err(js_err)?;
        let model =
            <Self as NotificationSettingsRepositoryExt>::update(self, identifier, &payload)
                .await?;
        to_js(&model)
    }
}

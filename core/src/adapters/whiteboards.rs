use chrono::Utc;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{self, whiteboards::ActiveModel};

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[ts(rename_all = "camelCase")]
#[ts(export, export_to = "whiteboards.ts")]
pub struct CreateWhiteboard {
    pub title: Option<String>,
    pub payload: Option<serde_json::Value>,
    pub workspace_identifier: Option<Uuid>,
}

impl Into<entities::whiteboards::ActiveModel> for CreateWhiteboard {
    fn into(self) -> entities::whiteboards::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            title: Set(self.title),
            payload: Set(self.payload),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
            workspace_identifier: Set(self.workspace_identifier),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[ts(rename_all = "camelCase")]
#[ts(export, export_to = "whiteboards.ts")]
pub struct UpdateWhiteboard {
    pub title: Option<String>,
    pub payload: Option<serde_json::Value>,
}

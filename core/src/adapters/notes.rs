use chrono::Utc;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

use crate::entities::{self, notes::ActiveModel};
use crate::id::new_id;

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[ts(rename_all = "camelCase")]
#[ts(export, export_to = "notes.ts")]
pub struct CreateNote {
    pub title: String,
    pub content: String,
    pub categories: Option<Vec<String>>,
    pub workspace_identifier: Option<String>,
}

impl Into<entities::notes::ActiveModel> for CreateNote {
    fn into(self) -> entities::notes::ActiveModel {
        let categories = serde_json::json!(self.categories.unwrap_or_default());
        ActiveModel {
            identifier: Set(new_id()),
            title: Set(self.title),
            content: Set(self.content),
            categories: Set(Some(categories)),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
            workspace_identifier: Set(self.workspace_identifier),
            deleted_at: Set(None),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[ts(rename_all = "camelCase")]
#[ts(export, export_to = "notes.ts")]
pub struct UpdateNote {
    pub title: Option<String>,
    pub content: Option<String>,
    pub categories: Option<Vec<String>>,
}

use chrono::Utc;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

use crate::entities::task_category::ActiveModel;
use crate::id::new_id;

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "task_category.ts")]
pub struct CreateTaskCategory {
    pub name: String,
    pub color: String,
}

impl Into<ActiveModel> for CreateTaskCategory {
    fn into(self) -> ActiveModel {
        ActiveModel {
            identifier: Set(new_id()),
            name: Set(self.name),
            color: Set(self.color),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "task_category.ts")]
pub struct UpdateTaskCategory {
    pub name: Option<String>,
    pub color: Option<String>,
}

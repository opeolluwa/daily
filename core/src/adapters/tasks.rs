use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Date;
use serde::{Deserialize, Serialize};

use crate::entities::{self, sea_orm_active_enums::Priority, task::ActiveModel};
use crate::id::new_id;

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "task.ts")]
pub struct CreateTask {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<Date>,
    pub priority: Priority,
    pub category_identifier: Option<String>,
}

impl Into<entities::task::ActiveModel> for CreateTask {
    fn into(self) -> entities::task::ActiveModel {
        ActiveModel {
            identifier: Set(new_id()),
            title: Set(self.title),
            description: Set(self.description),
            due_date: Set(self.due_date),
            priority: Set(self.priority),
            done: Set(false),
            category_identifier: Set(self.category_identifier),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
            deleted_at: Set(None),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "task.ts")]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub status: Option<String>,
    pub category_identifier: Option<Option<String>>,
}

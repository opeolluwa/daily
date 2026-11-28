use chrono::Utc;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

use crate::entities::{self, task, task_subtask};
use crate::id::new_id;

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "task_subtask.ts")]
pub struct CreateSubtask {
    pub title: String,
    pub sort_order: Option<i32>,
}

impl Into<entities::task_subtask::ActiveModel> for CreateSubtask {
    fn into(self) -> entities::task_subtask::ActiveModel {
        entities::task_subtask::ActiveModel {
            identifier: Set(new_id()),
            task_identifier: Set(String::new()),
            title: Set(self.title),
            done: Set(false),
            sort_order: Set(self.sort_order.unwrap_or(0)),
            workspace_identifier: Set(None),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
            deleted_at: Set(None),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "task_subtask.ts")]
pub struct UpdateSubtask {
    pub title: Option<String>,
    pub done: Option<bool>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "task_subtask.ts")]
pub struct SubtaskInput {
    pub identifier: Option<String>,
    pub title: String,
    pub done: Option<bool>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "task_subtask.ts")]
pub struct TaskWithSubtasks {
    pub task: task::Model,
    pub subtasks: Vec<task_subtask::Model>,
    pub progress: u8,
}

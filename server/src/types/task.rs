#![allow(non_snake_case)]
use kernel::entities;
use kernel::entities::sea_orm_active_enums::{Priority, TaskStatus};
use seaography::async_graphql;
use seaography::CustomInputType;
use serde::{Deserialize, Serialize};

#[derive(CustomInputType, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[seaography(input_type_name = "SyncTodoInput")]
pub struct SyncTodoInput {
    pub identifier: String,
    pub title: String,
    pub description: Option<String>,
    pub dueDate: Option<String>,
    pub priority: Priority,
    pub done: bool,
    pub createdAt: String,
    pub updatedAt: String,
    pub dueTime: Option<String>,
    #[serde(default)]
    pub status: Option<TaskStatus>,
    pub workspaceIdentifier: Option<String>,
    #[serde(default)]
    pub deletedAt: Option<String>,
}

impl TryFrom<SyncTodoInput> for entities::task::Model {
    type Error = async_graphql::Error;

    fn try_from(val: SyncTodoInput) -> Result<Self, Self::Error> {
        Ok(entities::task::Model {
            identifier: val.identifier,
            title: val.title,
            description: val.description,
            due_date: val
                .dueDate
                .map(|d| {
                    d.parse()
                        .map_err(|e| async_graphql::Error::new(format!("invalid due_date: {e}")))
                })
                .transpose()?,
            priority: val.priority,
            done: val.done,
            created_at: val
                .createdAt
                .parse()
                .map_err(|e| async_graphql::Error::new(format!("invalid created_at: {e}")))?,
            updated_at: val
                .updatedAt
                .parse()
                .map_err(|e| async_graphql::Error::new(format!("invalid updated_at: {e}")))?,
            due_time: val
                .dueTime
                .map(|t| {
                    t.parse()
                        .map_err(|e| async_graphql::Error::new(format!("invalid due_time: {e}")))
                })
                .transpose()?,
            workspace_identifier: val.workspaceIdentifier,
            status: val.status.unwrap_or(TaskStatus::Planned),
            deleted_at: val
                .deletedAt
                .map(|d| {
                    d.parse()
                        .map_err(|e| async_graphql::Error::new(format!("invalid deleted_at: {e}")))
                })
                .transpose()?,
        })
    }
}

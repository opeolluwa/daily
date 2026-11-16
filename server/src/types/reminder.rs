#![allow(non_snake_case)]
use kernel::entities;
use seaography::async_graphql;
use seaography::CustomInputType;
use serde::{Deserialize, Serialize};

#[derive(CustomInputType, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[seaography(input_type_name = "SyncReminderInput")]
pub struct SyncReminderInput {
    pub identifier: String,
    pub title: String,
    pub description: Option<String>,
    pub recurring: bool,
    pub recurrenceRule: Option<String>,
    pub alarmSound: Option<String>,
    pub remindAt: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub workspaceIdentifier: Option<String>,
    #[serde(default)]
    pub deletedAt: Option<String>,
}

impl TryFrom<SyncReminderInput> for entities::reminder::Model {
    type Error = async_graphql::Error;

    fn try_from(val: SyncReminderInput) -> Result<Self, Self::Error> {
        Ok(entities::reminder::Model {
            identifier: val.identifier,
            title: val.title,
            description: val.description,
            recurring: val.recurring,
            recurrence_rule: val.recurrenceRule,
            alarm_sound: val.alarmSound,
            deleted_at: val
                .deletedAt
                .map(|d| {
                    d.parse()
                        .map_err(|e| async_graphql::Error::new(format!("invalid deleted_at: {e}")))
                })
                .transpose()?,
            remind_at: val
                .remindAt
                .parse()
                .map_err(|e| async_graphql::Error::new(format!("invalid remind_at: {e}")))?,
            created_at: val
                .createdAt
                .parse()
                .map_err(|e| async_graphql::Error::new(format!("invalid created_at: {e}")))?,
            updated_at: val
                .updatedAt
                .parse()
                .map_err(|e| async_graphql::Error::new(format!("invalid updated_at: {e}")))?,
            workspace_identifier: val.workspaceIdentifier,
            recurrence_type: None,
            end_date: None,
        })
    }
}

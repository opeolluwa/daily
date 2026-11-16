#![allow(non_snake_case)]
use kernel::entities;
use seaography::async_graphql;
use seaography::CustomInputType;
use serde::{Deserialize, Serialize};

#[derive(CustomInputType, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[seaography(input_type_name = "SyncNoteInput")]
pub struct SyncNoteInput {
    pub identifier: String,
    pub title: String,
    pub content: String,
    pub categories: Option<serde_json::Value>,
    pub createdAt: String,
    pub updatedAt: String,
    pub workspaceIdentifier: Option<String>,
    #[serde(default)]
    pub deletedAt: Option<String>,
}

impl TryFrom<SyncNoteInput> for entities::notes::Model {
    type Error = async_graphql::Error;

    fn try_from(val: SyncNoteInput) -> Result<Self, Self::Error> {
        Ok(entities::notes::Model {
            identifier: val.identifier,
            title: val.title,
            content: val.content,
            categories: val.categories.map(sea_orm::JsonValue::from),
            created_at: val
                .createdAt
                .parse()
                .map_err(|e| async_graphql::Error::new(format!("invalid created_at: {e}")))?,
            updated_at: val
                .updatedAt
                .parse()
                .map_err(|e| async_graphql::Error::new(format!("invalid updated_at: {e}")))?,
            workspace_identifier: val.workspaceIdentifier,
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

#![allow(non_snake_case)]
use kernel::entities;
use seaography::async_graphql;
use seaography::CustomInputType;
use serde::{Deserialize, Serialize};

#[derive(CustomInputType, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[seaography(input_type_name = "SyncSnippetInput")]
pub struct SyncSnippetInput {
    pub identifier: String,
    pub title: Option<String>,
    pub language: Option<String>,
    pub code: String,
    pub description: Option<String>,
    pub isPinned: bool,
    pub createdAt: String,
    pub updatedAt: String,
    pub workspaceIdentifier: Option<String>,
    #[serde(default)]
    pub deletedAt: Option<String>,
}

impl TryFrom<SyncSnippetInput> for entities::snippets::Model {
    type Error = async_graphql::Error;

    fn try_from(val: SyncSnippetInput) -> Result<Self, Self::Error> {
        Ok(entities::snippets::Model {
            identifier: val.identifier,
            title: val.title,
            language: val.language,
            code: val.code,
            description: val.description,
            is_pinned: val.isPinned,
            deleted_at: val
                .deletedAt
                .map(|d| {
                    d.parse()
                        .map_err(|e| async_graphql::Error::new(format!("invalid deleted_at: {e}")))
                })
                .transpose()?,
            created_at: val
                .createdAt
                .parse()
                .map_err(|e| async_graphql::Error::new(format!("invalid created_at: {e}")))?,
            updated_at: val
                .updatedAt
                .parse()
                .map_err(|e| async_graphql::Error::new(format!("invalid updated_at: {e}")))?,
            workspace_identifier: val.workspaceIdentifier,
        })
    }
}

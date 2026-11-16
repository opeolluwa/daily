#![allow(non_snake_case)]
use kernel::entities;
use kernel::entities::sea_orm_active_enums::Tag;
use seaography::async_graphql;
use seaography::CustomInputType;
use serde::{Deserialize, Serialize};

#[derive(CustomInputType, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[seaography(input_type_name = "SyncBookmarkInput")]
pub struct SyncBookmarkInput {
    pub identifier: String,
    pub title: String,
    pub url: String,
    pub tag: Tag,
    pub createdAt: String,
    pub updatedAt: String,
    pub workspaceIdentifier: Option<String>,
    #[serde(default)]
    pub deletedAt: Option<String>,
}

impl TryFrom<SyncBookmarkInput> for entities::bookmark::Model {
    type Error = async_graphql::Error;

    fn try_from(val: SyncBookmarkInput) -> Result<Self, Self::Error> {
        Ok(entities::bookmark::Model {
            identifier: val.identifier,
            title: val.title,
            url: val.url,
            tag: val.tag,
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

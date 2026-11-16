#![allow(non_snake_case)]
use kernel::entities;
use seaography::async_graphql;
use seaography::CustomInputType;
use serde::{Deserialize, Serialize};

#[derive(CustomInputType, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[seaography(input_type_name = "SyncWorkspaceInput")]
pub struct SyncWorkspaceInput {
    pub identifier: String,
    pub name: String,
    pub description: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub isDefault: bool,
    pub isHidden: bool,
    pub isSecured: bool,
    pub passwordHash: Option<String>,
    pub userIdentifier: Option<String>,
    #[serde(default)]
    pub deletedAt: Option<String>,
}

impl TryFrom<SyncWorkspaceInput> for entities::workspaces::Model {
    type Error = async_graphql::Error;

    fn try_from(val: SyncWorkspaceInput) -> Result<Self, Self::Error> {
        Ok(entities::workspaces::Model {
            identifier: val.identifier,
            name: val.name,
            description: val.description,
            created_at: val
                .createdAt
                .parse()
                .map_err(|e| async_graphql::Error::new(format!("invalid created_at: {e}")))?,
            updated_at: val
                .updatedAt
                .parse()
                .map_err(|e| async_graphql::Error::new(format!("invalid updated_at: {e}")))?,
            is_default: val.isDefault,
            is_hidden: val.isHidden,
            is_secured: val.isSecured,
            password_hash: val.passwordHash,
            user_identifier: val.userIdentifier,
            deleted_at: val
                .deletedAt
                .map(|value| {
                    value.parse().map_err(|e| {
                        async_graphql::Error::new(format!("invalid deleted_at: {e}"))
                    })
                })
                .transpose()?,
        })
    }
}

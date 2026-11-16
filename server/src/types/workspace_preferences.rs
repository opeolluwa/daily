#![allow(non_snake_case)]
use kernel::entities;
use seaography::async_graphql;
use seaography::CustomInputType;
use serde::{Deserialize, Serialize};

#[derive(CustomInputType, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[seaography(input_type_name = "SyncWorkspacePreferenceInput")]
pub struct SyncWorkspacePreferenceInput {
    pub identifier: String,
    pub firstName: String,
    pub lastName: String,
    pub email: String,
    pub profilePicture: Option<String>,
    pub createdAt: String,
    pub updatedAt: String,
    pub workspaceIdentifier: Option<String>,
    #[serde(default)]
    pub deletedAt: Option<String>,
}

impl TryFrom<SyncWorkspacePreferenceInput> for entities::workspace_profiles::Model {
    type Error = async_graphql::Error;

    fn try_from(val: SyncWorkspacePreferenceInput) -> Result<Self, Self::Error> {
        Ok(entities::workspace_profiles::Model {
            identifier: val.identifier,
            first_name: val.firstName,
            last_name: val.lastName,
            profile_picture: val.profilePicture,
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

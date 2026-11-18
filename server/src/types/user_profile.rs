#![allow(non_snake_case)]
use kernel::entities;
use seaography::async_graphql;
use seaography::CustomInputType;
use serde::{Deserialize, Serialize};

#[derive(CustomInputType, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[seaography(input_type_name = "SyncUserProfileInput")]
pub struct SyncUserProfileInput {
    pub identifier: String,
    pub firstName: String,
    pub lastName: String,
    pub email: String,
    pub profilePicture: Option<String>,
    pub createdAt: String,
    pub updatedAt: String,
}

impl TryFrom<SyncUserProfileInput> for entities::user_profile::Model {
    type Error = async_graphql::Error;

    fn try_from(val: SyncUserProfileInput) -> Result<Self, Self::Error> {
        Ok(entities::user_profile::Model {
            identifier: val.identifier,
            first_name: val.firstName,
            last_name: val.lastName,
            email: val.email,
            profile_picture: val.profilePicture,
            created_at: val
                .createdAt
                .parse()
                .map_err(|e| async_graphql::Error::new(format!("invalid created_at: {e}")))?,
            updated_at: val
                .updatedAt
                .parse()
                .map_err(|e| async_graphql::Error::new(format!("invalid updated_at: {e}")))?,
        })
    }
}

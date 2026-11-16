#![allow(non_snake_case)]
use kernel::entities;
use kernel::entities::sea_orm_active_enums::ItemType;
use seaography::async_graphql;
use seaography::CustomInputType;
use serde::{Deserialize, Serialize};

#[derive(CustomInputType, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[seaography(input_type_name = "SyncRecycleBinInput")]
pub struct SyncRecycleBinInput {
    pub identifier: String,
    pub itemId: String,
    pub itemType: ItemType,
    pub payload: String,
    pub deletedAt: String,
    pub workspaceIdentifier: Option<String>,
}

impl TryFrom<SyncRecycleBinInput> for entities::recycle_bin::Model {
    type Error = async_graphql::Error;

    fn try_from(val: SyncRecycleBinInput) -> Result<Self, Self::Error> {
        Ok(entities::recycle_bin::Model {
            identifier: val.identifier,
            item_id: val.itemId,
            item_type: val.itemType,
            payload: val.payload,
            deleted_at: val
                .deletedAt
                .parse()
                .map_err(|e| async_graphql::Error::new(format!("invalid deleted_at: {e}")))?,
            workspace_identifier: val.workspaceIdentifier,
        })
    }
}

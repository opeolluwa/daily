use chrono::Utc;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

use crate::entities::{self, recycle_bin::ActiveModel, sea_orm_active_enums::ItemType};
use crate::id::new_id;

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "recycle_bin.ts")]
pub struct CreateRecycleBinEntry {
    pub item_id: String,
    pub item_type: ItemType,
    pub payload: String,
    pub workspace_identifier: Option<String>,
}

impl Into<entities::recycle_bin::ActiveModel> for CreateRecycleBinEntry {
    fn into(self) -> entities::recycle_bin::ActiveModel {
        ActiveModel {
            identifier: Set(new_id()),
            item_id: Set(self.item_id),
            item_type: Set(self.item_type),
            payload: Set(self.payload),
            workspace_identifier: Set(self.workspace_identifier),
            deleted_at: Set(Utc::now().fixed_offset()),
        }
    }
}

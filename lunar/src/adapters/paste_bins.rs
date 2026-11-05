use sea_orm::ActiveValue::Set;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{self, paste_bins::ActiveModel};

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[ts(rename_all = "camelCase")]
#[ts(export, export_to = "paste_bins.ts")]
pub struct CreatePasteBin {
    pub title: Option<String>,
    pub content: String,
    pub is_pinned: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub workspace_identifier: Option<Uuid>,
}

impl Into<entities::paste_bins::ActiveModel> for CreatePasteBin {
    fn into(self) -> entities::paste_bins::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            title: Set(self.title),
            content: Set(self.content),
            is_pinned: Set(self.is_pinned),
            created_at: Set(self.created_at),
            updated_at: Set(self.updated_at),
            workspace_identifier: Set(self.workspace_identifier),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[ts(rename_all = "camelCase")]
#[ts(export, export_to = "paste_bins.ts")]
pub struct UpdatePasteBin {
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_pinned: Option<bool>,
}

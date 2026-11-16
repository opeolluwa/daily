use sea_orm::ActiveValue::Set;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};

use crate::entities::{self, snippets::ActiveModel};
use crate::id::new_id;

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[deprecated(since = "0.1.0", note = "use CreateSnippet ")]
#[ts(rename_all = "camelCase")]
#[ts(export, export_to = "snippets.ts")]
pub struct Snippet {
    pub title: Option<String>,
    pub language: Option<String>,
    pub code: String,
    pub description: Option<String>,
    pub is_pinned: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[ts(rename_all = "camelCase")]
#[ts(export, export_to = "snippets.ts")]
pub struct CreateSnippet {
    pub title: Option<String>,
    pub language: Option<String>,
    pub code: String,
    pub description: Option<String>,
    pub is_pinned: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub workspace_identifier: Option<String>,
}

impl Into<entities::snippets::ActiveModel> for CreateSnippet {
    fn into(self) -> entities::snippets::ActiveModel {
        ActiveModel {
            identifier: Set(new_id()),
            title: Set(self.title),
            language: Set(self.language),
            code: Set(self.code),
            description: Set(self.description),
            is_pinned: Set(self.is_pinned),
            created_at: Set(self.created_at),
            updated_at: Set(self.updated_at),
            workspace_identifier: Set(self.workspace_identifier),
            deleted_at: Set(None),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[ts(rename_all = "camelCase")]
#[ts(export, export_to = "snippets.ts")]
pub struct UpdateSnippet {
    pub title: Option<String>,
    pub language: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub is_pinned: Option<bool>,
}

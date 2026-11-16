use chrono::Utc;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

use crate::entities::{self, bookmark::ActiveModel, sea_orm_active_enums::Tag};
use crate::id::new_id;

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "bookmark.ts")]
pub struct CreateBookmark {
    pub title: String,
    pub url: String,
    pub tag: Tag,
}

impl Into<entities::bookmark::ActiveModel> for CreateBookmark {
    fn into(self) -> entities::bookmark::ActiveModel {
        ActiveModel {
            identifier: Set(new_id()),
            title: Set(self.title),
            url: Set(self.url),
            tag: Set(self.tag),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
            deleted_at: Set(None),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "bookmarks.ts")]
pub struct UpdateBookmark {
    pub title: Option<String>,
    pub url: Option<String>,
    pub tag: Option<Tag>,
}

use kernel::{adapters::meta::RequestMeta, sea_orm::sea_query::prelude::Local};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePasteBin {
    pub title: Option<String>,
    pub content: String,
    #[serde(default)]
    pub is_pinned: bool,
    pub workspace_identifier: Option<Uuid>,
    pub meta: Option<RequestMeta>,
}

impl From<CreatePasteBin> for kernel::adapters::paste_bins::CreatePasteBin {
    fn from(paste_bin: CreatePasteBin) -> Self {
        Self {
            title: paste_bin.title,
            content: paste_bin.content,
            is_pinned: paste_bin.is_pinned,
            created_at: Local::now().into(),
            updated_at: Local::now().into(),
            workspace_identifier: paste_bin.workspace_identifier,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePasteBin {
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_pinned: Option<bool>,
    pub meta: Option<RequestMeta>,
}

impl From<UpdatePasteBin> for kernel::adapters::paste_bins::UpdatePasteBin {
    fn from(paste_bin: UpdatePasteBin) -> Self {
        Self {
            title: paste_bin.title,
            content: paste_bin.content,
            is_pinned: paste_bin.is_pinned,
        }
    }
}

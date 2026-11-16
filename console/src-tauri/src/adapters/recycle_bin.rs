use kernel::adapters::meta::RequestMeta;
use kernel::entities::sea_orm_active_enums::ItemType as RecycleBinItemType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecycleBinEntry {
    pub item_id: String,
    pub item_type: RecycleBinItemType,
    pub payload: String,
    pub workspace_identifier: Option<String>,
    pub meta: Option<RequestMeta>,
}

impl From<CreateRecycleBinEntry> for kernel::adapters::recycle_bin::CreateRecycleBinEntry {
    fn from(e: CreateRecycleBinEntry) -> Self {
        Self {
            item_id: e.item_id,
            item_type: e.item_type,
            payload: e.payload,
            workspace_identifier: e.workspace_identifier,
        }
    }
}

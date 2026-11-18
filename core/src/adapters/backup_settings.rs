use serde::{Deserialize, Serialize};

use crate::entities::sea_orm_active_enums::BackupMode;

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "backup_settings.ts")]
pub struct UpdateBackupSettings {
    pub server_url: Option<String>,
    pub backup_mode: Option<BackupMode>,
}

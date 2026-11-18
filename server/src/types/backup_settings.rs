#![allow(non_snake_case)]
use kernel::entities;
use kernel::entities::sea_orm_active_enums::BackupMode;
use seaography::async_graphql;
use seaography::CustomInputType;
use serde::{Deserialize, Serialize};

#[derive(CustomInputType, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[seaography(input_type_name = "SyncBackupSettingsInput")]
pub struct SyncBackupSettingsInput {
    pub identifier: String,
    pub userIdentifier: Option<String>,
    pub serverUrl: String,
    pub lastSync: Option<String>,
    pub backupMode: String,
    pub createdAt: String,
    pub updatedAt: String,
}

impl TryFrom<SyncBackupSettingsInput> for entities::backup_settings::Model {
    type Error = async_graphql::Error;

    fn try_from(val: SyncBackupSettingsInput) -> Result<Self, Self::Error> {
        let backup_mode = match val.backupMode.as_str() {
            "local_only" => BackupMode::LocalOnly,
            "cloud_only" => BackupMode::CloudOnly,
            "local_first" => BackupMode::LocalFirst,
            "cloud_first" => BackupMode::CloudFirst,
            other => {
                return Err(async_graphql::Error::new(format!(
                    "invalid backup_mode: {other}"
                )))
            }
        };

        let last_sync = val
            .lastSync
            .as_deref()
            .map(|s| {
                s.parse()
                    .map_err(|e| async_graphql::Error::new(format!("invalid last_sync: {e}")))
            })
            .transpose()?;

        Ok(entities::backup_settings::Model {
            identifier: val.identifier,
            user_identifier: val.userIdentifier,
            server_url: val.serverUrl,
            last_sync,
            backup_mode,
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

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "notification_settings.ts")]
pub struct UpdateNotificationSettings {
    pub tasks_due: Option<bool>,
    pub reminders: Option<bool>,
    pub sync_complete: Option<bool>,
    pub app_updates: Option<bool>,
}

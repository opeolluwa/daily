use chrono::Utc;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

use crate::entities::{self, user_profile::ActiveModel};
use crate::id::new_id;

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "user_profile.ts")]
pub struct CreateUserProfile {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub profile_picture: Option<String>,
}

impl Into<entities::user_profile::ActiveModel> for CreateUserProfile {
    fn into(self) -> entities::user_profile::ActiveModel {
        ActiveModel {
            identifier: Set(new_id()),
            first_name: Set(self.first_name),
            last_name: Set(self.last_name),
            email: Set(self.email),
            profile_picture: Set(self.profile_picture),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "user_profile.ts")]
pub struct UpdateUserProfile {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub profile_picture: Option<String>,
}

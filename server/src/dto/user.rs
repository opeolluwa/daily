use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserEntity {
    pub email: String,
    pub identifier: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: String,
    pub is_active: bool,
    pub profile_picture: Option<String>,
    pub username: Option<String>,
}

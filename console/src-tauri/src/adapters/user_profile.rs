use kernel::adapters::user_profile as kernel_adapters;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserProfile {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub profile_picture: Option<String>,
}

impl From<CreateUserProfile> for kernel_adapters::CreateUserProfile {
    fn from(val: CreateUserProfile) -> Self {
        Self {
            first_name: val.first_name,
            last_name: val.last_name,
            email: val.email,
            profile_picture: val.profile_picture,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserProfile {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub profile_picture: Option<String>,
}

impl From<UpdateUserProfile> for kernel_adapters::UpdateUserProfile {
    fn from(val: UpdateUserProfile) -> Self {
        Self {
            first_name: val.first_name,
            last_name: val.last_name,
            email: val.email,
            profile_picture: val.profile_picture,
        }
    }
}

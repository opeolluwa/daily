use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::sea_orm_active_enums::BackupMode;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "backup_settings")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub identifier: String,
    #[sea_orm(column_type = "Char(Some(36))", nullable)]
    pub user_identifier: Option<String>,
    pub server_url: String,
    pub last_sync: Option<DateTimeWithTimeZone>,
    pub backup_mode: BackupMode,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {}

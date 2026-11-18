use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "notification_settings")]
#[serde(rename_all = "camelCase")]
#[kernel_macros::ts_rs_export_sea_orm_entity_name]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub identifier: String,
    pub tasks_due: bool,
    pub reminders: bool,
    pub sync_complete: bool,
    pub app_updates: bool,
    #[sea_orm(column_type = "Char(Some(36))", nullable)]
    pub user_identifier: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {}

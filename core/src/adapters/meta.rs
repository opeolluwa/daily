use sanitizer::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Sanitizer, Validate, Clone, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "meta.ts")]

pub struct RequestMeta {
    pub workspace_identifier: String,
}

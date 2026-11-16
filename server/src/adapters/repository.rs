use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "repository.ts")]

pub struct DatabaseInsertResult {
    pub identifier: String,
}

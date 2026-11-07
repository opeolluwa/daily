use std::{env, str::FromStr, sync::Arc};

use sea_orm::{DatabaseBackend, DatabaseConnection, MockDatabase};
use wasm_bindgen::JsValue;

use crate::{adapters::meta::RequestMeta, error::LunarError};

pub(crate) fn extract_req_meta(meta: &Option<RequestMeta>) -> Result<RequestMeta, LunarError> {
    let Some(meta) = meta else {
        return Err(LunarError::DbConnectError(
            "missing workspace identifier".into(),
        ));
    };

    Ok(meta.to_owned())
}

/// Checks whether a file already exists at the given path.
///
/// File-related operations should call this before writing so the caller can
/// decide whether to overwrite, rename, or abort.
#[cfg(not(target_arch = "wasm32"))]
pub async fn detect_duplicate(
    path: impl AsRef<std::path::Path>,
) -> std::io::Result<Option<std::path::PathBuf>> {
    let path = path.as_ref();

    if tokio::fs::try_exists(path).await? {
        Ok(Some(path.to_path_buf()))
    } else {
        Ok(None)
    }
}

pub fn mock_connection() -> Arc<DatabaseConnection> {
    Arc::new(MockDatabase::new(DatabaseBackend::Postgres).into_connection())
}

pub fn to_js<T: serde::Serialize>(value: &T) -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(value).map_err(|e| JsValue::from_str(&e.to_string()))
}

pub fn js_err(error: impl ToString) -> JsValue {
    JsValue::from_str(&error.to_string())
}

impl From<LunarError> for JsValue {
    fn from(error: LunarError) -> Self {
        serde_wasm_bindgen::to_value(&error)
            .unwrap_or_else(|_| JsValue::from_str(&error.to_string()))
    }
}

pub fn extract_env<T: FromStr>(env_key: &str) -> Result<T, LunarError> {
    let env = env::var(env_key)
        .map_err(|_| {
            log::error!("error fetching env {}", env_key);
            LunarError::EnvError(env_key.to_string())
        })?
        .parse::<T>()
        .map_err(|_| {
            log::error!("error parsing env due to");
            LunarError::EnvError("error parsing env".into())
        })?;

    Ok(env)
}

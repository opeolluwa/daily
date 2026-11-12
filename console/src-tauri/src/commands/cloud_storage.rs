use kernel::filestore::{
    FileStoreConfig, FileStoreEngine, FileStoreProvider, ImagekitConfig, S3Config, StorageBackend,
};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{errors::AppError, state::app::AppState};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudStorageRequest {
    pub provider: String,
    pub imagekit: Option<ImagekitConfigRequest>,
    pub s3: Option<S3ConfigRequest>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagekitConfigRequest {
    pub public_key: String,
    pub private_key: String,
    pub upload_url: String,
    pub folder: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S3ConfigRequest {
    pub bucket: String,
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub endpoint: Option<String>,
    pub public_base_url: Option<String>,
    pub key_prefix: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudStorageStatus {
    pub active: bool,
    pub provider: Option<String>,
}

fn build_config(request: &CloudStorageRequest) -> Result<FileStoreConfig, AppError> {
    let backend = match request.provider.as_str() {
        "imagekit" => StorageBackend::Imagekit,
        "s3" => StorageBackend::S3,
        other => {
            return Err(AppError::Io(format!(
                "unknown cloud storage provider: {other}"
            )))
        }
    };

    let imagekit = request
        .imagekit
        .as_ref()
        .map(|c| ImagekitConfig {
            public_key: SecretString::from(c.public_key.clone()),
            private_key: SecretString::from(c.private_key.clone()),
            upload_url: c.upload_url.clone(),
            folder: c.folder.clone(),
        })
        .unwrap_or_else(|| ImagekitConfig {
            public_key: SecretString::from(String::new()),
            private_key: SecretString::from(String::new()),
            upload_url: String::new(),
            folder: None,
        });

    let s3 = request
        .s3
        .as_ref()
        .map(|c| S3Config {
            bucket: c.bucket.clone(),
            region: c.region.clone(),
            access_key_id: SecretString::from(c.access_key_id.clone()),
            secret_access_key: SecretString::from(c.secret_access_key.clone()),
            session_token: None,
            endpoint: c.endpoint.clone(),
            force_path_style: c.endpoint.is_some(),
            public_base_url: c.public_base_url.clone(),
            key_prefix: c.key_prefix.clone(),
        })
        .unwrap_or_else(|| S3Config {
            bucket: String::new(),
            region: String::new(),
            access_key_id: SecretString::from(String::new()),
            secret_access_key: SecretString::from(String::new()),
            session_token: None,
            endpoint: None,
            force_path_style: false,
            public_base_url: None,
            key_prefix: None,
        });

    Ok(FileStoreConfig {
        backend,
        imagekit,
        s3,
    })
}

#[tauri::command]
pub async fn configure_cloud_storage(
    state: State<'_, AppState>,
    config: CloudStorageRequest,
) -> Result<CloudStorageStatus, AppError> {
    let file_store_config = build_config(&config)?;
    let engine =
        FileStoreEngine::from_config(&file_store_config).map_err(|e| AppError::Io(e.to_string()))?;

    let provider = engine.backend().to_string();
    *state.cloud_storage.write().await = Some(engine);

    Ok(CloudStorageStatus {
        active: true,
        provider: Some(provider),
    })
}

#[tauri::command]
pub async fn get_cloud_storage_status(
    state: State<'_, AppState>,
) -> Result<CloudStorageStatus, AppError> {
    let guard = state.cloud_storage.read().await;
    match guard.as_ref() {
        Some(engine) => Ok(CloudStorageStatus {
            active: true,
            provider: Some(engine.backend().to_string()),
        }),
        None => Ok(CloudStorageStatus {
            active: false,
            provider: None,
        }),
    }
}

#[tauri::command]
pub async fn disable_cloud_storage(
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    *state.cloud_storage.write().await = None;
    Ok(())
}

#[tauri::command]
pub async fn test_cloud_connection(
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let guard = state.cloud_storage.read().await;
    let engine = guard
        .as_ref()
        .ok_or_else(|| AppError::Io("cloud storage is not configured".into()))?;

    let test_content = b"daily-cloud-connection-test";
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("daily-cloud-test.txt");
    std::fs::write(&test_file, test_content).map_err(|e| AppError::Io(e.to_string()))?;

    let url = engine
        .upload_file(&test_file, "daily-cloud-test.txt", Some("text/plain"))
        .await
        .map_err(|e| AppError::Io(e.to_string()))?;

    let _ = std::fs::remove_file(&test_file);

    Ok(url)
}

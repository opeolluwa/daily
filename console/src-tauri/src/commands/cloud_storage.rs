use std::io::Write;
use std::path::{Path, PathBuf};

use kernel::filestore::{
    FileStoreConfig, FileStoreEngine, FileStoreProvider, ImagekitConfig, S3Config, StorageBackend,
};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};

use crate::{errors::AppError, secrets, state::app::AppState};

const METADATA_FILE: &str = "cloud-storage.json";

/// Cloud configuration sent from the webview.
///
/// Secret fields are optional: leaving them empty keeps the value already
/// stored in the OS keychain. Secrets are never sent back to the webview.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudStorageRequest {
    pub provider: String,
    pub imagekit: Option<ImagekitConfigRequest>,
    pub s3: Option<S3ConfigRequest>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagekitConfigRequest {
    pub public_key: String,
    #[serde(default)]
    pub private_key: String,
    pub upload_url: String,
    pub folder: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S3ConfigRequest {
    pub bucket: String,
    pub region: String,
    #[serde(default)]
    pub access_key_id: String,
    #[serde(default)]
    pub secret_access_key: String,
    pub endpoint: Option<String>,
    pub public_base_url: Option<String>,
    pub key_prefix: Option<String>,
}

/// Non-secret configuration, safe to persist and to return to the webview.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudStorageConfigPublic {
    pub provider: String,
    pub imagekit: Option<ImagekitConfigPublic>,
    pub s3: Option<S3ConfigPublic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagekitConfigPublic {
    pub public_key: String,
    pub upload_url: String,
    pub folder: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S3ConfigPublic {
    pub bucket: String,
    pub region: String,
    pub endpoint: Option<String>,
    pub public_base_url: Option<String>,
    pub key_prefix: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StoredCloudStorage {
    enabled: bool,
    #[serde(flatten)]
    config: CloudStorageConfigPublic,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudStorageStatus {
    pub active: bool,
    pub enabled: bool,
    pub provider: Option<String>,
    pub has_credentials: bool,
    pub config: Option<CloudStorageConfigPublic>,
}

struct ResolvedSecrets {
    imagekit_private_key: Option<String>,
    s3_access_key_id: Option<String>,
    s3_secret_access_key: Option<String>,
}

fn non_empty(value: &str) -> Option<&str> {
    (!value.is_empty()).then_some(value)
}

fn resolve_secrets(request: &CloudStorageRequest) -> Result<ResolvedSecrets, AppError> {
    match request.provider.as_str() {
        "imagekit" => {
            let config = request
                .imagekit
                .as_ref()
                .ok_or_else(|| AppError::Io("ImageKit configuration is required".into()))?;

            let private_key = match non_empty(&config.private_key) {
                Some(value) => value.to_owned(),
                None => secrets::get(secrets::IMAGEKIT_PRIVATE_KEY)?
                    .ok_or_else(|| AppError::Secrets("ImageKit private key is required".into()))?,
            };

            Ok(ResolvedSecrets {
                imagekit_private_key: Some(private_key),
                s3_access_key_id: None,
                s3_secret_access_key: None,
            })
        }
        "s3" => {
            let config = request
                .s3
                .as_ref()
                .ok_or_else(|| AppError::Io("S3 configuration is required".into()))?;

            let access_key_id = match non_empty(&config.access_key_id) {
                Some(value) => value.to_owned(),
                None => secrets::get(secrets::S3_ACCESS_KEY_ID)?
                    .ok_or_else(|| AppError::Secrets("S3 access key ID is required".into()))?,
            };

            let secret_access_key = match non_empty(&config.secret_access_key) {
                Some(value) => value.to_owned(),
                None => secrets::get(secrets::S3_SECRET_ACCESS_KEY)?
                    .ok_or_else(|| AppError::Secrets("S3 secret access key is required".into()))?,
            };

            Ok(ResolvedSecrets {
                imagekit_private_key: None,
                s3_access_key_id: Some(access_key_id),
                s3_secret_access_key: Some(secret_access_key),
            })
        }
        other => Err(AppError::Io(format!(
            "unknown cloud storage provider: {other}"
        ))),
    }
}

fn build_config(
    request: &CloudStorageRequest,
    resolved: &ResolvedSecrets,
) -> Result<FileStoreConfig, AppError> {
    let backend = match request.provider.as_str() {
        "imagekit" => StorageBackend::Imagekit,
        "s3" => StorageBackend::S3,
        other => {
            return Err(AppError::Io(format!(
                "unknown cloud storage provider: {other}"
            )))
        }
    };

    let imagekit = match (&request.imagekit, backend) {
        (Some(config), StorageBackend::Imagekit) => ImagekitConfig {
            public_key: SecretString::from(config.public_key.clone()),
            private_key: SecretString::from(
                resolved.imagekit_private_key.clone().unwrap_or_default(),
            ),
            upload_url: config.upload_url.clone(),
            folder: config.folder.clone(),
        },
        _ => ImagekitConfig {
            public_key: SecretString::from(String::new()),
            private_key: SecretString::from(String::new()),
            upload_url: String::new(),
            folder: None,
        },
    };

    let s3 = match (&request.s3, backend) {
        (Some(config), StorageBackend::S3) => S3Config {
            bucket: config.bucket.clone(),
            region: config.region.clone(),
            access_key_id: SecretString::from(
                resolved.s3_access_key_id.clone().unwrap_or_default(),
            ),
            secret_access_key: SecretString::from(
                resolved.s3_secret_access_key.clone().unwrap_or_default(),
            ),
            session_token: None,
            endpoint: config.endpoint.clone(),
            force_path_style: config.endpoint.is_some(),
            public_base_url: config.public_base_url.clone(),
            key_prefix: config.key_prefix.clone(),
        },
        _ => S3Config {
            bucket: String::new(),
            region: String::new(),
            access_key_id: SecretString::from(String::new()),
            secret_access_key: SecretString::from(String::new()),
            session_token: None,
            endpoint: None,
            force_path_style: false,
            public_base_url: None,
            key_prefix: None,
        },
    };

    Ok(FileStoreConfig {
        backend,
        imagekit,
        s3,
    })
}

fn public_config(request: &CloudStorageRequest) -> CloudStorageConfigPublic {
    if request.provider == "imagekit" {
        CloudStorageConfigPublic {
            provider: "imagekit".into(),
            imagekit: request
                .imagekit
                .as_ref()
                .map(|config| ImagekitConfigPublic {
                    public_key: config.public_key.clone(),
                    upload_url: config.upload_url.clone(),
                    folder: config.folder.clone(),
                }),
            s3: None,
        }
    } else {
        CloudStorageConfigPublic {
            provider: "s3".into(),
            imagekit: None,
            s3: request.s3.as_ref().map(|config| S3ConfigPublic {
                bucket: config.bucket.clone(),
                region: config.region.clone(),
                endpoint: config.endpoint.clone(),
                public_base_url: config.public_base_url.clone(),
                key_prefix: config.key_prefix.clone(),
            }),
        }
    }
}

fn request_from_stored(stored: &StoredCloudStorage) -> CloudStorageRequest {
    CloudStorageRequest {
        provider: stored.config.provider.clone(),
        imagekit: stored
            .config
            .imagekit
            .as_ref()
            .map(|config| ImagekitConfigRequest {
                public_key: config.public_key.clone(),
                private_key: String::new(),
                upload_url: config.upload_url.clone(),
                folder: config.folder.clone(),
            }),
        s3: stored.config.s3.as_ref().map(|config| S3ConfigRequest {
            bucket: config.bucket.clone(),
            region: config.region.clone(),
            access_key_id: String::new(),
            secret_access_key: String::new(),
            endpoint: config.endpoint.clone(),
            public_base_url: config.public_base_url.clone(),
            key_prefix: config.key_prefix.clone(),
        }),
    }
}

fn metadata_path(app: &AppHandle) -> Result<PathBuf, AppError> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|error| AppError::Io(error.to_string()))?;

    Ok(dir.join(METADATA_FILE))
}

fn write_metadata(app: &AppHandle, stored: &StoredCloudStorage) -> Result<(), AppError> {
    let path = metadata_path(app)?;
    let contents =
        serde_json::to_vec_pretty(stored).map_err(|error| AppError::Io(error.to_string()))?;

    std::fs::write(&path, contents).map_err(|error| AppError::Io(error.to_string()))?;
    restrict_permissions(&path)?;

    Ok(())
}

#[cfg(unix)]
fn restrict_permissions(path: &Path) -> Result<(), AppError> {
    use std::os::unix::fs::PermissionsExt;

    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))
        .map_err(|error| AppError::Io(error.to_string()))
}

#[cfg(not(unix))]
fn restrict_permissions(_path: &Path) -> Result<(), AppError> {
    Ok(())
}

fn read_metadata(app: &AppHandle) -> Option<StoredCloudStorage> {
    let path = metadata_path(app).ok()?;
    let bytes = std::fs::read(path).ok()?;

    serde_json::from_slice(&bytes).ok()
}

fn credential_present(provider: &str) -> bool {
    let account = match provider {
        "imagekit" => secrets::IMAGEKIT_PRIVATE_KEY,
        "s3" => secrets::S3_SECRET_ACCESS_KEY,
        _ => return false,
    };

    matches!(secrets::get(account), Ok(Some(_)))
}

async fn current_status(app: &AppHandle, state: &AppState) -> CloudStorageStatus {
    let engine_provider = {
        let guard = state.cloud_storage.read().await;
        guard.as_ref().map(|engine| engine.backend().to_string())
    };

    let active = engine_provider.is_some();
    let stored = read_metadata(app);

    CloudStorageStatus {
        active,
        enabled: stored.as_ref().is_some_and(|stored| stored.enabled),
        provider: engine_provider
            .or_else(|| stored.as_ref().map(|stored| stored.config.provider.clone())),
        has_credentials: stored
            .as_ref()
            .is_some_and(|stored| credential_present(&stored.config.provider)),
        config: stored.map(|stored| stored.config),
    }
}

/// Loads the saved configuration into the in-memory engine at startup.
///
/// Runs before the window is shown so whiteboard uploads work without opening
/// the settings screen first. Credential-store failures are logged and leave
/// cloud backup inactive; secrets are never written to the log.
pub async fn load_saved_storage(app: &AppHandle, state: &AppState) {
    let Some(stored) = read_metadata(app) else {
        return;
    };

    if !stored.enabled {
        return;
    }

    let request = request_from_stored(&stored);

    let resolved = match resolve_secrets(&request) {
        Ok(resolved) => resolved,
        Err(error) => {
            log::warn!("cloud storage credentials are unavailable: {error}");
            return;
        }
    };

    let config = match build_config(&request, &resolved) {
        Ok(config) => config,
        Err(error) => {
            log::warn!("stored cloud storage configuration is invalid: {error}");
            return;
        }
    };

    match FileStoreEngine::from_config(&config) {
        Ok(engine) => {
            *state.cloud_storage.write().await = Some(engine);
        }
        Err(error) => log::warn!("stored cloud storage configuration is invalid: {error}"),
    }
}

#[tauri::command]
pub async fn save_cloud_storage(
    app: AppHandle,
    state: State<'_, AppState>,
    config: CloudStorageRequest,
) -> Result<CloudStorageStatus, AppError> {
    let resolved = resolve_secrets(&config)?;
    let file_store_config = build_config(&config, &resolved)?;

    match config.provider.as_str() {
        "imagekit" => {
            if let Some(private_key) = &resolved.imagekit_private_key {
                secrets::set(secrets::IMAGEKIT_PRIVATE_KEY, private_key)?;
            }

            // Drop credentials that belong to the provider we just switched away
            // from instead of leaving unused keys in the keychain.
            secrets::delete(secrets::S3_ACCESS_KEY_ID)?;
            secrets::delete(secrets::S3_SECRET_ACCESS_KEY)?;
        }
        "s3" => {
            if let Some(access_key_id) = &resolved.s3_access_key_id {
                secrets::set(secrets::S3_ACCESS_KEY_ID, access_key_id)?;
            }

            if let Some(secret_access_key) = &resolved.s3_secret_access_key {
                secrets::set(secrets::S3_SECRET_ACCESS_KEY, secret_access_key)?;
            }

            secrets::delete(secrets::IMAGEKIT_PRIVATE_KEY)?;
        }
        other => {
            return Err(AppError::Io(format!(
                "unknown cloud storage provider: {other}"
            )))
        }
    }

    let engine = FileStoreEngine::from_config(&file_store_config)
        .map_err(|error| AppError::Io(error.to_string()))?;

    let stored = StoredCloudStorage {
        enabled: true,
        config: public_config(&config),
    };

    write_metadata(&app, &stored)?;
    *state.cloud_storage.write().await = Some(engine);

    Ok(current_status(&app, &state).await)
}

#[tauri::command]
pub async fn get_cloud_storage_status(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<CloudStorageStatus, AppError> {
    Ok(current_status(&app, &state).await)
}

#[tauri::command]
pub async fn disable_cloud_storage(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<CloudStorageStatus, AppError> {
    *state.cloud_storage.write().await = None;

    if let Some(mut stored) = read_metadata(&app) {
        stored.enabled = false;
        write_metadata(&app, &stored)?;
    }

    Ok(current_status(&app, &state).await)
}

/// Deletes every stored credential and the non-secret configuration.
#[tauri::command]
pub async fn clear_cloud_storage(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<CloudStorageStatus, AppError> {
    secrets::delete_cloud_storage()?;

    if let Ok(path) = metadata_path(&app) {
        let _ = std::fs::remove_file(path);
    }

    *state.cloud_storage.write().await = None;

    Ok(current_status(&app, &state).await)
}

/// Verifies credentials without persisting them.
///
/// Empty secret fields fall back to the values already in the keychain.
#[tauri::command]
pub async fn test_cloud_storage(config: CloudStorageRequest) -> Result<String, AppError> {
    let resolved = resolve_secrets(&config)?;
    let file_store_config = build_config(&config, &resolved)?;
    let engine = FileStoreEngine::from_config(&file_store_config)
        .map_err(|error| AppError::Io(error.to_string()))?;

    let mut file =
        tempfile::NamedTempFile::new().map_err(|error| AppError::Io(error.to_string()))?;
    file.write_all(b"daily-cloud-connection-test")
        .map_err(|error| AppError::Io(error.to_string()))?;
    file.flush()
        .map_err(|error| AppError::Io(error.to_string()))?;

    engine
        .upload_file(file.path(), "daily-cloud-test.txt", Some("text/plain"))
        .await
        .map_err(|error| AppError::Io(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn imagekit_request() -> CloudStorageRequest {
        CloudStorageRequest {
            provider: "imagekit".into(),
            imagekit: Some(ImagekitConfigRequest {
                public_key: "public_abc".into(),
                private_key: "private-secret".into(),
                upload_url: "https://upload.imagekit.io/api/v1/files/upload".into(),
                folder: Some("/whiteboards".into()),
            }),
            s3: None,
        }
    }

    fn s3_request() -> CloudStorageRequest {
        CloudStorageRequest {
            provider: "s3".into(),
            imagekit: None,
            s3: Some(S3ConfigRequest {
                bucket: "my-bucket".into(),
                region: "us-east-1".into(),
                access_key_id: "AKIAEXAMPLE".into(),
                secret_access_key: "s3-secret".into(),
                endpoint: Some("https://s3.example.com".into()),
                public_base_url: Some("https://cdn.example.com".into()),
                key_prefix: Some("uploads/".into()),
            }),
        }
    }

    #[test]
    fn public_config_drops_imagekit_private_key() {
        let serialized = serde_json::to_string(&public_config(&imagekit_request())).unwrap();

        assert!(serialized.contains("public_abc"));
        assert!(!serialized.contains("private-secret"));
    }

    #[test]
    fn public_config_drops_s3_credentials() {
        let serialized = serde_json::to_string(&public_config(&s3_request())).unwrap();

        assert!(serialized.contains("my-bucket"));
        assert!(!serialized.contains("AKIAEXAMPLE"));
        assert!(!serialized.contains("s3-secret"));
    }

    #[test]
    fn request_from_stored_never_carries_secrets() {
        let stored = StoredCloudStorage {
            enabled: true,
            config: public_config(&s3_request()),
        };

        let request = request_from_stored(&stored);
        let s3 = request.s3.expect("s3 config");

        assert_eq!(request.provider, "s3");
        assert!(s3.access_key_id.is_empty());
        assert!(s3.secret_access_key.is_empty());
        assert_eq!(s3.bucket, "my-bucket");
    }

    #[test]
    fn stored_config_roundtrips_through_json() {
        let stored = StoredCloudStorage {
            enabled: true,
            config: public_config(&imagekit_request()),
        };

        let json = serde_json::to_string(&stored).unwrap();
        let restored: StoredCloudStorage = serde_json::from_str(&json).unwrap();

        assert!(restored.enabled);
        assert_eq!(restored.config.provider, "imagekit");
        assert_eq!(
            restored
                .config
                .imagekit
                .expect("imagekit config")
                .public_key,
            "public_abc"
        );
    }
}

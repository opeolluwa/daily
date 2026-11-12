use keyring::{Entry, Error as KeyringError};

use crate::errors::AppError;

/// Service name used for every credential this app stores in the OS keychain.
const SERVICE: &str = "com.opeolluwa.daily";

pub const IMAGEKIT_PRIVATE_KEY: &str = "cloud-storage.imagekit.private-key";
pub const S3_ACCESS_KEY_ID: &str = "cloud-storage.s3.access-key-id";
pub const S3_SECRET_ACCESS_KEY: &str = "cloud-storage.s3.secret-access-key";

fn entry(account: &str) -> Result<Entry, AppError> {
    Entry::new(SERVICE, account).map_err(map_error)
}

/// Stores a credential in the OS keychain, replacing any existing value.
pub fn set(account: &str, value: &str) -> Result<(), AppError> {
    entry(account)?.set_password(value).map_err(map_error)
}

/// Reads a credential, returning `None` when it has never been stored.
pub fn get(account: &str) -> Result<Option<String>, AppError> {
    match entry(account)?.get_password() {
        Ok(value) => Ok(Some(value)),
        Err(KeyringError::NoEntry) => Ok(None),
        Err(error) => Err(map_error(error)),
    }
}

/// Removes a credential. Missing entries are treated as already removed.
pub fn delete(account: &str) -> Result<(), AppError> {
    match entry(account)?.delete_credential() {
        Ok(()) | Err(KeyringError::NoEntry) => Ok(()),
        Err(error) => Err(map_error(error)),
    }
}

pub fn delete_cloud_storage() -> Result<(), AppError> {
    delete(IMAGEKIT_PRIVATE_KEY)?;
    delete(S3_ACCESS_KEY_ID)?;
    delete(S3_SECRET_ACCESS_KEY)?;
    Ok(())
}

fn map_error(error: KeyringError) -> AppError {
    AppError::Secrets(error.to_string())
}

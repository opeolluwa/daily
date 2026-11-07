use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, thiserror::Error)]
pub enum LunarError {
    #[error("failed to connect to database due to {0}")]
    DbConnectError(String),

    #[error("database operation failed: {0}")]
    DbOperationError(String),

    #[error("Failed to extract env: {0}")]
    EnvError(String),

    #[error("Workspace does not exist: {0}")]
    WorkspaceNotFound(String),

    #[error("Bookmark does not exist: {0}")]
    BookmarkNotFound(String),

    #[error("Note does not exist: {0}")]
    NotesNotFound(String),

    #[error("Todo does not exist: {0}")]
    TodoNotFound(String),

    #[error("Snippet does not exist: {0}")]
    SnippetNotFound(String),

    #[error("Paste bin does not exist: {0}")]
    PasteBinNotFound(String),

    #[error("Reminder does not exists ")]
    ReminderNotFound(String),

    #[error("Notification does not exist: {0}")]
    NotificationNotFound(String),

    #[cfg(feature = "pdf")]
    #[error("Failed to generate PDF: {0}")]
    HTMLConversionError(String),

    #[cfg(feature = "pdf")]
    #[error("File already exists: {0}")]
    FileExists(String),

    #[cfg(feature = "cipher")]
    #[error("Cipher key decode failed: {0}")]
    CipherKeyDecodeError(String),

    #[cfg(feature = "cipher")]
    #[error("Cipher encryption failed: {0}")]
    CipherEncryptError(String),

    #[cfg(feature = "cipher")]
    #[error("Cipher decryption failed: {0}")]
    CipherDecryptError(String),
}

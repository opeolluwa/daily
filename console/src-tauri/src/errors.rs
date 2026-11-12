use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0}")]
    Kernel(#[from] kernel::error::KernelError),
    #[error("IO error: {0}")]
    Io(String),
    #[error("Credential store error: {0}")]
    Secrets(String),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

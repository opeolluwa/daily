use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum IntelligenceError {
    #[error("Provider error: {0}")]
    Provider(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Provider not available: {0}")]
    Unavailable(String),

    #[error("Stream closed unexpectedly")]
    StreamClosed,
}

impl Serialize for IntelligenceError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

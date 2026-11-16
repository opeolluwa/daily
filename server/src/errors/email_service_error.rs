#[derive(thiserror::Error, Debug)]
pub enum EmailServiceError {
    #[error("failed to send email")]
    SendEmailError,
    #[error("failed due to {0}")]
    OperationFailed(String),
    #[error("failed due to {0}")]
    ProviderError(String),
    #[error("failed due to {0}")]
    DeliveryError(String),
}

#[cfg(feature = "ollama")]
pub mod ollama;

use tokio::sync::mpsc;

use crate::config::{ProviderConfig, ProviderKind};
use crate::error::IntelligenceError;
use crate::types::{ChatMessage, CompletionResponse, StreamEvent};

pub enum ProviderImpl {
    #[cfg(feature = "ollama")]
    Ollama(ollama::OllamaProvider),
}

impl ProviderImpl {
    pub async fn complete(
        &self,
        model: &str,
        messages: &[ChatMessage],
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> Result<CompletionResponse, IntelligenceError> {
        match self {
            #[cfg(feature = "ollama")]
            Self::Ollama(p) => p.complete(model, messages, temperature, max_tokens).await,
        }
    }

    pub async fn stream(
        &self,
        model: &str,
        messages: &[ChatMessage],
        temperature: Option<f32>,
        max_tokens: Option<u32>,
        tx: mpsc::Sender<StreamEvent>,
    ) -> Result<(), IntelligenceError> {
        match self {
            #[cfg(feature = "ollama")]
            Self::Ollama(p) => p.stream(model, messages, temperature, max_tokens, tx).await,
        }
    }

    pub async fn health_check(&self) -> Result<bool, IntelligenceError> {
        match self {
            #[cfg(feature = "ollama")]
            Self::Ollama(p) => p.health_check().await,
        }
    }

    pub async fn list_models(&self) -> Result<Vec<String>, IntelligenceError> {
        match self {
            #[cfg(feature = "ollama")]
            Self::Ollama(p) => p.list_models().await,
        }
    }
}

pub fn create_provider(config: &ProviderConfig) -> Result<ProviderImpl, IntelligenceError> {
    match config.kind {
        ProviderKind::Ollama => {
            #[cfg(feature = "ollama")]
            {
                Ok(ProviderImpl::Ollama(ollama::OllamaProvider::new(config)?))
            }
            #[cfg(not(feature = "ollama"))]
            {
                Err(IntelligenceError::Unavailable(
                    "Ollama support not compiled".into(),
                ))
            }
        }
        ProviderKind::OpenAi => Err(IntelligenceError::Unavailable(
            "OpenAI provider not yet implemented".into(),
        )),
        ProviderKind::Anthropic => Err(IntelligenceError::Unavailable(
            "Anthropic provider not yet implemented".into(),
        )),
    }
}

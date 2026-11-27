use tokio::sync::mpsc;

use crate::config::ProviderConfig;
use crate::error::IntelligenceError;
use crate::providers::{self, ProviderImpl};
use crate::types::{ChatMessage, CompletionResponse, StreamEvent};

pub struct IntelligenceClient {
    provider: ProviderImpl,
    config: ProviderConfig,
}

impl IntelligenceClient {
    pub fn new(config: ProviderConfig) -> Result<Self, IntelligenceError> {
        let provider = providers::create_provider(&config)?;
        Ok(Self { provider, config })
    }

    pub async fn complete(
        &self,
        messages: &[ChatMessage],
    ) -> Result<CompletionResponse, IntelligenceError> {
        self.provider
            .complete(&self.config.model, messages, None, None)
            .await
    }

    pub async fn complete_with(
        &self,
        messages: &[ChatMessage],
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> Result<CompletionResponse, IntelligenceError> {
        self.provider
            .complete(&self.config.model, messages, temperature, max_tokens)
            .await
    }

    pub async fn stream(
        &self,
        messages: &[ChatMessage],
        tx: mpsc::Sender<StreamEvent>,
    ) -> Result<(), IntelligenceError> {
        self.provider
            .stream(&self.config.model, messages, None, None, tx)
            .await
    }

    pub async fn health_check(&self) -> Result<bool, IntelligenceError> {
        self.provider.health_check().await
    }

    pub async fn list_models(&self) -> Result<Vec<String>, IntelligenceError> {
        self.provider.list_models().await
    }

    pub fn config(&self) -> &ProviderConfig {
        &self.config
    }

    pub fn reconfigure(&mut self, config: ProviderConfig) -> Result<(), IntelligenceError> {
        let provider = providers::create_provider(&config)?;
        self.provider = provider;
        self.config = config;
        Ok(())
    }
}

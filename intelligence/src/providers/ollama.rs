use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::chat::ChatMessage as OllamaChatMessage;
use ollama_rs::Ollama;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

use crate::config::ProviderConfig;
use crate::error::IntelligenceError;
use crate::types::{ChatMessage, CompletionResponse, Role, StreamEvent};

pub struct OllamaProvider {
    client: Ollama,
}

impl OllamaProvider {
    pub fn new(config: &ProviderConfig) -> Result<Self, IntelligenceError> {
        let endpoint = config
            .endpoint
            .as_deref()
            .unwrap_or("http://localhost:11434");

        let parsed = url::Url::parse(endpoint)
            .map_err(|e| IntelligenceError::Config(format!("Invalid endpoint URL: {e}")))?;

        let host = format!(
            "{}://{}",
            parsed.scheme(),
            parsed.host_str().unwrap_or("localhost")
        );
        let port = parsed.port().unwrap_or(11434);

        Ok(Self {
            client: Ollama::builder().host(host).port(port).build(),
        })
    }

    pub async fn complete(
        &self,
        model: &str,
        messages: &[ChatMessage],
        _temperature: Option<f32>,
        _max_tokens: Option<u32>,
    ) -> Result<CompletionResponse, IntelligenceError> {
        let request = ChatMessageRequest::new(model.to_string(), to_ollama_messages(messages));

        let response = self
            .client
            .send_chat_messages(request)
            .await
            .map_err(|e| IntelligenceError::Provider(e.to_string()))?;

        let content = response.message.content;

        Ok(CompletionResponse {
            message: ChatMessage::assistant(content),
            model: model.to_string(),
            usage: None,
        })
    }

    pub async fn stream(
        &self,
        model: &str,
        messages: &[ChatMessage],
        _temperature: Option<f32>,
        _max_tokens: Option<u32>,
        tx: mpsc::Sender<StreamEvent>,
    ) -> Result<(), IntelligenceError> {
        let request = ChatMessageRequest::new(model.to_string(), to_ollama_messages(messages));

        let mut stream = self
            .client
            .send_chat_messages_stream(request)
            .await
            .map_err(|e| IntelligenceError::Provider(e.to_string()))?;

        while let Some(Ok(chunk)) = stream.next().await {
            if !chunk.message.content.is_empty() {
                let _ = tx
                    .send(StreamEvent::Delta {
                        content: chunk.message.content.clone(),
                    })
                    .await;
            }

            if chunk.done {
                let _ = tx
                    .send(StreamEvent::Done {
                        model: model.to_string(),
                        usage: None,
                    })
                    .await;
                return Ok(());
            }
        }

        let _ = tx
            .send(StreamEvent::Done {
                model: model.to_string(),
                usage: None,
            })
            .await;

        Ok(())
    }

    pub async fn health_check(&self) -> Result<bool, IntelligenceError> {
        self.client
            .list_local_models()
            .await
            .map(|_| true)
            .map_err(|e| IntelligenceError::Provider(e.to_string()))
    }

    pub async fn list_models(&self) -> Result<Vec<String>, IntelligenceError> {
        let models = self
            .client
            .list_local_models()
            .await
            .map_err(|e| IntelligenceError::Provider(e.to_string()))?;

        Ok(models.into_iter().map(|m| m.name).collect())
    }
}

fn to_ollama_messages(messages: &[ChatMessage]) -> Vec<OllamaChatMessage> {
    messages
        .iter()
        .map(|m| match m.role {
            Role::System => OllamaChatMessage::system(m.content.clone()),
            Role::User => OllamaChatMessage::user(m.content.clone()),
            Role::Assistant => OllamaChatMessage::assistant(m.content.clone()),
        })
        .collect()
}

use tokio::sync::mpsc;

use crate::client::IntelligenceClient;
use crate::context::AppContext;
use crate::error::IntelligenceError;
use crate::types::{ChatMessage, CompletionResponse, Conversation, StreamEvent};

pub const DEFAULT_SYSTEM_PROMPT: &str = "You are a helpful assistant integrated into the Daily productivity app. You help the user manage their notes, tasks, bookmarks, and reminders. Be concise and practical. When the user's data is provided as context, reference it naturally.";

pub struct Agent {
    client: IntelligenceClient,
    system_prompt: String,
    conversation: Conversation,
}

impl Agent {
    pub fn new(client: IntelligenceClient, model: &str) -> Self {
        Self {
            system_prompt: DEFAULT_SYSTEM_PROMPT.to_string(),
            conversation: Conversation::new(model),
            client,
        }
    }

    pub fn with_context(mut self, context: &AppContext) -> Self {
        let context_section = context.to_system_prompt_section();
        if !context_section.is_empty() {
            self.system_prompt = format!(
                "{}\n\n---\n\nHere is the user's current data:\n\n{}",
                self.system_prompt, context_section
            );
        }
        self
    }

    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = prompt.into();
        self
    }

    fn build_messages(&self) -> Vec<ChatMessage> {
        let mut messages = vec![ChatMessage::system(&self.system_prompt)];
        messages.extend(self.conversation.messages.clone());
        messages
    }

    pub async fn send(
        &mut self,
        user_message: impl Into<String>,
    ) -> Result<CompletionResponse, IntelligenceError> {
        let user_msg = ChatMessage::user(user_message);
        self.conversation.push(user_msg);

        let messages = self.build_messages();
        let response = self.client.complete(&messages).await?;

        self.conversation.push(response.message.clone());
        Ok(response)
    }

    pub async fn send_streaming(
        &mut self,
        user_message: impl Into<String>,
        tx: mpsc::Sender<StreamEvent>,
    ) -> Result<(), IntelligenceError> {
        let user_msg = ChatMessage::user(user_message);
        self.conversation.push(user_msg);

        let messages = self.build_messages();
        self.client.stream(&messages, tx).await
    }

    pub fn conversation(&self) -> &Conversation {
        &self.conversation
    }

    pub fn reset(&mut self) {
        self.conversation = Conversation::new(&self.conversation.model);
    }
}

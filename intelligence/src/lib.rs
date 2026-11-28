pub mod agent;
pub mod client;
pub mod config;
pub mod context;
pub mod error;
pub mod providers;
pub mod types;

pub use client::IntelligenceClient;
pub use config::{ProviderConfig, ProviderKind};
pub use context::{AppContext, ContextItem};
pub use error::IntelligenceError;
pub use types::{ChatMessage, CompletionResponse, Conversation, Role, StreamEvent, UsageStats};

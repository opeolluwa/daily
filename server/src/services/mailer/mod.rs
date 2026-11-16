pub mod engine;
pub mod health;
pub mod mailtrap;
pub mod queued;
pub mod sanitize;
pub mod smtp;
pub mod templates;

use serde::{Deserialize, Serialize};

use crate::errors::email_service_error::EmailServiceError;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmailCategory {
    #[default]
    Account,
    Transactions,
    Support,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmailAttachment {
    pub filename: String,
    pub content_type: String,
    /// Base64-encoded file content.
    pub data: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmailContext {
    pub from_address: String,
    pub from_name: String,
    pub to_address: String,
    pub to_name: String,
    pub subject: String,
    pub html_body: String,
    #[serde(default)]
    pub category: EmailCategory,
    #[serde(default)]
    pub attachments: Vec<EmailAttachment>,
}

/// Seeds the sender identity, leaves the per-message fields empty.
///
/// The dispatch layer runs without access to the config, so it builds messages
/// with `..EmailContext::default()` and lets the queue-backed sender swap in the
/// deployment's real identity. Defaulting the identity here — rather than
/// deriving `Default` and starting from empty strings — is what keeps a message
/// from ever reaching a provider with a blank `from_address`, which Mailtrap
/// rejects outright.
impl Default for EmailContext {
    fn default() -> Self {
        let from = SenderIdentity::default();
        Self {
            from_address: from.address,
            from_name: from.name,
            to_address: String::new(),
            to_name: String::new(),
            subject: String::new(),
            html_body: String::new(),
            category: EmailCategory::default(),
            attachments: Vec::new(),
        }
    }
}

// Only ever used inside this crate, where every future is already required to
// be `Send` by the queue worker; the sugar is not hiding a leaky bound.
#[allow(async_fn_in_trait)]
#[cfg_attr(test, mockall::automock)]
pub trait EmailSender {
    async fn send_email(&self, message: EmailContext) -> Result<(), EmailServiceError>;
}

/// The From address and display name for outgoing mail.
///
/// [`EmailContext::default`] carries this identity onto every message the
/// dispatch layer builds, and the queue-backed sender rewrites those two fields
/// with the deployment's configured identity on its way into the job payload.
/// Applying it there rather than at the dispatch layer keeps the identity out of
/// the `EmailSender` trait, which is what lets the mocked sender stay free of it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SenderIdentity {
    pub address: String,
    pub name: String,
}

impl Default for SenderIdentity {
    fn default() -> Self {
        Self {
            address: "noreply@almonds.app".to_string(),
            name: "Almonds".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SenderIdentities {
    pub account: SenderIdentity,
    pub transactions: SenderIdentity,
    pub support: SenderIdentity,
}

impl SenderIdentities {
    pub fn get(&self, category: &EmailCategory) -> &SenderIdentity {
        match category {
            EmailCategory::Account => &self.account,
            EmailCategory::Transactions => &self.transactions,
            EmailCategory::Support => &self.support,
        }
    }
}

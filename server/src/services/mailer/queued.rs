use bullmq::{
    types::{BackoffStrategy, RemoveOnFinish},
    Queue,
};

use super::{EmailContext, EmailSender, SenderIdentities};
use crate::{
    errors::email_service_error::EmailServiceError,
    services::redis::{
        producer::{EMAIL_JOB_ATTEMPTS, EMAIL_JOB_BACKOFF_MILLIS},
        EMAIL_JOB_NAME,
    },
};

/// Hands emails to the queue instead of delivering them inline.
///
/// The `send-email` worker is what actually talks to the provider, so a slow or
/// briefly unreachable server no longer blocks the request that triggered the
/// mail.
#[derive(Clone)]
pub struct QueuedEmailSender {
    queue: Queue,
    identities: SenderIdentities,
}

impl QueuedEmailSender {
    pub fn new(queue: Queue, identities: SenderIdentities) -> Self {
        Self { queue, identities }
    }
}

impl EmailSender for QueuedEmailSender {
    /// Stamps the deployment's configured identity onto the message before it
    /// becomes a job payload.
    ///
    /// The dispatch layer runs without access to the config, so it fills in
    /// [`SenderIdentity::default`] and the real identity is applied here. This
    /// matters for the API provider, which rejects any sending domain the
    /// account has not verified. The identity is chosen by the message's
    /// [`EmailCategory`](super::EmailCategory): account, transactions, or
    /// support.
    async fn send_email(&self, mut message: EmailContext) -> Result<(), EmailServiceError> {
        let identity = self.identities.get(&message.category);
        message.from_address = identity.address.clone();
        message.from_name = identity.name.clone();

        self.queue
            .add(EMAIL_JOB_NAME, message)
            .attempts(EMAIL_JOB_ATTEMPTS)
            .backoff(BackoffStrategy::Exponential(EMAIL_JOB_BACKOFF_MILLIS))
            .remove_on_complete(RemoveOnFinish::Bool(true))
            .await
            .map(|_| ())
            .map_err(|err| {
                EmailServiceError::OperationFailed(format!("failed to enqueue email: {err}"))
            })
    }
}

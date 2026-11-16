use std::{sync::OnceLock, time::Duration};

use bullmq::{types::RemoveOnFinish, Job, Worker, WorkerOptions};

use crate::config::{AppConfig, Environment};
use crate::errors::email_service_error::EmailServiceError;
use crate::services::mailer::{engine::EmailEngine, EmailContext, EmailSender};
use crate::services::redis::{
    connection_options, CONNECT_ATTEMPTS, CONNECT_RETRY_DELAY, EMAIL_JOB_NAME, EMAIL_QUEUE_NAME,
};

/// Wall-clock ceiling for a single delivery attempt.
///
/// The worker can only book a retry while it still owns the job's Redis lock,
/// so this has to stay under [`EMAIL_JOB_LOCK_DURATION`].
pub const EMAIL_SEND_DEADLINE: Duration = Duration::from_secs(20);

/// How long BullMQ holds a job's lock before the stalled check may reclaim it.
///
/// A lock that expires mid-job is unrecoverable for that worker: the retry
/// script is rejected with `Job lock does not exist` and the job is stranded in
/// `active` with nobody left to book its next attempt. BullMQ defaults this to
/// 30s, which a slow relay can outlast, so it is raised well past the worst-case
/// delivery bounded by [`EMAIL_SEND_DEADLINE`].
///
/// Renewal runs on a `lock_duration / 2` ticker, so a 90s lock tolerates three
/// consecutive missed renewals.
pub const EMAIL_JOB_LOCK_DURATION: Duration = Duration::from_secs(90);

/// BullMQ stops a worker as soon as it is dropped, so the long-lived worker has
/// to outlive the task that starts it.
static EMAIL_WORKER: OnceLock<Worker> = OnceLock::new();

/// Starts the mail worker.
///
/// Every job is a serialized [`EmailContext`]; the worker only delivers it and
/// reports the outcome back to BullMQ, which owns the retry schedule.
pub async fn start_worker(config: AppConfig) {
    let sender = match EmailEngine::from_config(&config) {
        Ok(sender) => sender,
        Err(err) => {
            tracing::error!("{EMAIL_QUEUE_NAME} worker not started: {err}");
            return;
        }
    };

    let options = WorkerOptions::default()
        .connection(connection_options(&config))
        .lock_duration(EMAIL_JOB_LOCK_DURATION)
        // A single expired lock should cost the job one attempt, not the whole
        // job: the default of 1 sends a recovered job straight to `failed`.
        .max_stalled_count(2)
        .remove_on_complete(RemoveOnFinish::Bool(matches!(
            &config.environment,
            Environment::Production
        )));

    for attempt in 1..=CONNECT_ATTEMPTS {
        let processor = sender.clone();

        let worker = Worker::with_options(
            EMAIL_QUEUE_NAME,
            move |job: Job, _token| {
                let sender = processor.clone();

                async move { deliver(&sender, &job).await }
            },
            options.clone(),
        )
        .await;

        match worker {
            Ok(worker) => {
                if EMAIL_WORKER.set(worker).is_err() {
                    tracing::warn!("{EMAIL_QUEUE_NAME} worker already running");
                } else {
                    tracing::info!("{EMAIL_QUEUE_NAME} worker started");
                }

                return;
            }
            Err(err) => {
                tracing::warn!(
                    "failed to start {EMAIL_QUEUE_NAME} worker (attempt {attempt}/{CONNECT_ATTEMPTS}): {err}"
                );
                tokio::time::sleep(CONNECT_RETRY_DELAY).await;
            }
        }
    }

    tracing::error!("{EMAIL_QUEUE_NAME} worker gave up connecting to redis");
}

async fn deliver(sender: &EmailEngine, job: &Job) -> Result<serde_json::Value, bullmq::Error> {
    deliver_within(sender, job, EMAIL_SEND_DEADLINE).await
}

async fn deliver_within(
    sender: &EmailEngine,
    job: &Job,
    deadline: Duration,
) -> Result<serde_json::Value, bullmq::Error> {
    let message: EmailContext = serde_json::from_value(job.data().clone()).map_err(|err| {
        bullmq::Error::Unrecoverable(format!("invalid {EMAIL_JOB_NAME} job payload: {err}"))
    })?;

    tracing::info!("{EMAIL_QUEUE_NAME} job {} delivering mail", job.id());

    send_within(sender, message, deadline).await?;

    Ok(serde_json::json!({ "sent": true }))
}

/// Sends the mail under a wall-clock deadline.
///
/// Each provider bounds a single command round trip, so a server that accepts a
/// connection and then stalls can consume several of those budgets back to back.
/// Without an outer bound the send can outrun the job's Redis lock, and the
/// worker loses the right to book a retry. Timing out here reports a retryable
/// error while the lock is still held.
async fn send_within(
    sender: &EmailEngine,
    message: EmailContext,
    deadline: Duration,
) -> Result<(), bullmq::Error> {
    let send = sender.send_email(message);

    match tokio::time::timeout(deadline, send).await {
        Ok(Ok(())) => Ok(()),
        Ok(Err(err)) => Err(classify(err)),
        // The provider outlived its budget, but the job itself is still sound.
        Err(_) => Err(bullmq::Error::ProcessingError(format!(
            "delivery exceeded its {deadline:?} budget"
        ))),
    }
}

fn classify(err: EmailServiceError) -> bullmq::Error {
    match err {
        // A malformed address will not become valid, so retrying cannot help.
        EmailServiceError::OperationFailed(_) | EmailServiceError::ProviderError(_) => {
            bullmq::Error::Unrecoverable(err.to_string())
        }
        EmailServiceError::SendEmailError | EmailServiceError::DeliveryError(_) => {
            bullmq::Error::ProcessingError(err.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use bullmq::Job;
    use tokio::{net::TcpListener, task::JoinHandle};

    use super::*;
    use crate::{
        config::SmtpTlsMode,
        services::mailer::{mailtrap, smtp, smtp::SmtpEmailSender, EmailCategory},
    };

    fn message() -> EmailContext {
        EmailContext {
            from_address: "noreply@almonds.app".to_string(),
            from_name: "Almonds".to_string(),
            to_address: "member@example.com".to_string(),
            to_name: "Member".to_string(),
            subject: "Welcome".to_string(),
            html_body: "<p>hi</p>".to_string(),
            category: EmailCategory::default(),
            attachments: Vec::new(),
        }
    }

    fn job_with(data: serde_json::Value) -> Job {
        Job::new(EMAIL_JOB_NAME, data, None)
    }

    fn message_job() -> Job {
        job_with(serde_json::to_value(message()).expect("message serialises"))
    }

    /// The worker no longer builds its own transport, so the tests construct the
    /// SMTP engine directly to reach the relay they need to control.
    fn smtp_engine(host: &str, port: u16) -> EmailEngine {
        let sender =
            SmtpEmailSender::new(host, port, "", "", SmtpTlsMode::None).expect("builds a sender");

        EmailEngine::Smtp(sender)
    }

    /// A relay that accepts the TCP connection and then says nothing, so lettre
    /// waits on the greeting. This is the shape of stall that used to outlive the
    /// job's Redis lock.
    ///
    /// The accept loop has to stay non-blocking: `#[tokio::test]` runs on a
    /// single-threaded runtime, so a blocking `accept` would starve the very
    /// timer the delivery budget depends on.
    async fn stalling_relay() -> (SocketAddr, JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("binds an ephemeral port");
        let addr = listener.local_addr().expect("has a local address");

        let handle = tokio::spawn(async move {
            // Hold every accepted connection open without ever greeting, so the
            // client keeps waiting for a response that will not arrive.
            let mut held = Vec::new();

            while let Ok((stream, _)) = listener.accept().await {
                held.push(stream);
            }
        });

        (addr, handle)
    }

    /// A relay that greets and closes, enough for the SMTP sender to consider the
    /// message handed off.
    ///
    /// `accepting_relay` rather than a stub sender so the success path exercises
    /// the same code the worker runs in production.
    async fn accepting_relay() -> (SocketAddr, JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("binds an ephemeral port");
        let addr = listener.local_addr().expect("has a local address");

        let handle = tokio::spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

                let (reader, mut writer) = stream.into_split();
                // `into_split` hands back a bare `AsyncRead`, so the line reader
                // the SMTP conversation needs has to wrap it.
                let mut reader = BufReader::new(reader);
                let mut line = String::new();

                // Greet, then answer every command with a success so the sender
                // can finish the envelope exchange.
                let _ = writer.write_all(b"220 relay\r\n").await;
                let _ = writer.flush().await;

                while reader.read_line(&mut line).await.unwrap_or(0) > 0 {
                    let command = line.trim().to_ascii_uppercase();
                    line.clear();

                    let reply = match command.as_str() {
                        _ if command.starts_with("EHLO") || command.starts_with("HELO") => {
                            "250-relay\r\n250 OK\r\n"
                        }
                        "QUIT" => "221 Bye\r\n",
                        "MAIL FROM" | "RCPT TO" => "250 OK\r\n",
                        "DATA" => "354 Send it\r\n",
                        "RSET" | "NOOP" => "250 OK\r\n",
                        _ => "250 OK\r\n",
                    };

                    if writer.write_all(reply.as_bytes()).await.is_err() {
                        break;
                    }

                    let _ = writer.flush().await;

                    if command == "QUIT" {
                        break;
                    }
                }
            }
        });

        (addr, handle)
    }

    /// The ordering that keeps a failure recordable. These budgets drifted
    /// together at 30s, which is what let a send outlive its own lock.
    ///
    /// Every provider carries its own round-trip budget, so each one has to sit
    /// inside the delivery deadline or a single command can eat all of it.
    #[test]
    fn delivery_budgets_are_ordered_inside_the_lock_duration() {
        assert!(
            smtp::DEFAULT_SEND_TIMEOUT < EMAIL_SEND_DEADLINE,
            "a single SMTP command must not be able to consume the whole delivery budget"
        );
        assert!(
            mailtrap::DEFAULT_SEND_TIMEOUT < EMAIL_SEND_DEADLINE,
            "a single API round trip must not be able to consume the whole delivery budget"
        );
        assert!(
            EMAIL_SEND_DEADLINE < EMAIL_JOB_LOCK_DURATION,
            "delivery must finish well before the job lock can expire"
        );
    }

    #[tokio::test]
    async fn a_stalling_relay_times_out_while_the_lock_is_still_held() {
        let (addr, relay) = stalling_relay().await;
        let sender = smtp_engine(&addr.ip().to_string(), addr.port());

        let result = deliver_within(&sender, &message_job(), Duration::from_millis(200)).await;

        relay.abort();

        // Retryable, not a hard failure: the job is sound, the relay is not.
        assert!(
            matches!(result, Err(bullmq::Error::ProcessingError(_))),
            "expected a retryable timeout, got {result:?}"
        );
    }

    #[tokio::test]
    async fn an_unreachable_relay_is_retryable() {
        // Port 1 on loopback has nothing listening, so the send fails fast.
        let sender = smtp_engine("127.0.0.1", 1);

        let result = deliver_within(&sender, &message_job(), Duration::from_secs(5)).await;

        assert!(
            matches!(result, Err(bullmq::Error::ProcessingError(_))),
            "expected a retryable transport failure, got {result:?}"
        );
    }

    #[tokio::test]
    async fn an_unparseable_payload_is_not_retried() {
        let sender = smtp_engine("127.0.0.1", 1);

        let result = deliver_within(
            &sender,
            &job_with(serde_json::json!({})),
            Duration::from_secs(5),
        )
        .await;

        assert!(
            matches!(result, Err(bullmq::Error::Unrecoverable(_))),
            "a malformed payload cannot be fixed by retrying, got {result:?}"
        );
    }

    #[tokio::test]
    async fn a_delivered_job_reports_success() {
        let (addr, relay) = accepting_relay().await;
        let sender = smtp_engine(&addr.ip().to_string(), addr.port());

        let result = deliver_within(&sender, &message_job(), Duration::from_secs(10)).await;

        relay.abort();

        assert!(
            result.is_ok(),
            "expected the send to succeed, got {result:?}"
        );
    }
}

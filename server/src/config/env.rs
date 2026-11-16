use std::{env, str::FromStr};

use dotenv::dotenv;
use kernel::{error::KernelError, utils::extract_env};
use secrecy::SecretString;
use tower_http::cors::AllowOrigin;

use crate::errors::app_error::AppError;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub max_db_connections: u32,
    pub body_limit_mb: usize,
    pub upload_path: String,
    pub export_path: String,
    pub port: u16,
    pub allowed_origins: AllowOrigin,
    pub allowed_origin_strings: Vec<String>,
    pub environment: Environment,
    // GraphQL / API settings
    pub graphql_endpoint: String,
    pub depth_limit: Option<usize>,
    pub complexity_limit: Option<usize>,

    pub base_url: String,

    // Rate limiting
    pub rate_limit_rps: u32,
    pub rate_limit_burst: u32,
    pub rate_limit_rpm: u32,
    pub rate_limit_max_keys: usize,

    // Timeout
    pub timeout_secs: u64,

    // SMTP
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: SecretString,
    pub smtp_tls_mode: SmtpTlsMode,

    // Email provider
    pub email_provider: EmailProvider,
    pub mailtrap_api_token: SecretString,
    pub mailtrap_base_url: String,
    pub email_from_address: String,
    pub email_from_name: String,
    pub email_transactions_from_address: String,
    pub email_transactions_from_name: String,
    pub email_support_from_address: String,
    pub email_support_from_name: String,

    // Queue
    pub redis_url: SecretString,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Development,
    Production,
}

impl FromStr for Environment {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dev" | "development" => Ok(Environment::Development),
            "prod" | "production" => Ok(Environment::Production),
            _ => Err(AppError::InvalidEnv(s.to_string())),
        }
    }
}

impl Environment {
    pub fn is_production(&self) -> bool {
        matches!(self, Environment::Production)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EmailProvider {
    #[default]
    Smtp,
    Mailtrap,
}

impl FromStr for EmailProvider {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "smtp" | "lettre" => Ok(Self::Smtp),
            "mailtrap" | "api" => Ok(Self::Mailtrap),
            _ => Err(format!("Invalid email provider: {value}")),
        }
    }
}

impl std::fmt::Display for EmailProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Smtp => f.write_str("smtp"),
            Self::Mailtrap => f.write_str("mailtrap"),
        }
    }
}

/// `none` is plaintext and only valid outside production; it exists so local
/// development can point at a plain SMTP sink such as mailpit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SmtpTlsMode {
    /// No TLS at all. Rejected when `ENVIRONMENT=production`.
    None,

    /// Upgrade with STARTTLS, failing if the server does not support it.
    #[default]
    Starttls,

    /// Upgrade with STARTTLS when offered, silently falling back to plaintext.
    Opportunistic,

    /// Implicit TLS (SMTPS) from the first byte.
    Implicit,
}

impl FromStr for SmtpTlsMode {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "none" | "plain" | "plaintext" => Ok(Self::None),
            "starttls" | "required" => Ok(Self::Starttls),
            "opportunistic" => Ok(Self::Opportunistic),
            "implicit" | "wrapper" | "tls" => Ok(Self::Implicit),
            _ => Err(format!("Invalid SMTP TLS mode: {value}")),
        }
    }
}

fn default_email_provider() -> EmailProvider {
    EmailProvider::default()
}

fn default_smtp_tls_mode() -> SmtpTlsMode {
    SmtpTlsMode::default()
}

fn default_email_from_address() -> String {
    "noreply@almonds.app".to_string()
}

fn default_email_from_name() -> String {
    "Almonds".to_string()
}

fn default_email_transactions_from_address() -> String {
    "billing@almonds.app".to_string()
}

fn default_email_transactions_from_name() -> String {
    "Almonds Billing".to_string()
}

fn default_email_support_from_address() -> String {
    "support@almonds.app".to_string()
}

fn default_email_support_from_name() -> String {
    "Almonds Support".to_string()
}

fn default_mailtrap_base_url() -> String {
    "https://send.api.mailtrap.io/api/send".to_string()
}

fn parse_env<T: FromStr>(env_key: &str, default: T) -> Result<T, KernelError>
where
    T::Err: std::fmt::Display,
{
    match env::var(env_key) {
        Err(_) => Ok(default),
        Ok(raw) => {
            // `.env` files in this repo carry inline comments after enum values
            // (e.g. `SMTP_TLS_MODE=none # prod: starttls`). dotenv drops a
            // space-separated comment, but not one written tight against the
            // value, so strip a trailing `#...` before parsing.
            let value = raw.split('#').next().unwrap_or_default().trim();

            value.parse().map_err(|err| {
                KernelError::EnvError(format!("{env_key} has an invalid value: {err}"))
            })
        }
    }
}

impl std::fmt::Debug for AppConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppConfig")
            .field("database_url", &"****")
            .field("max_db_connections", &self.max_db_connections)
            .field("body_limit_mb", &self.body_limit_mb)
            .field("upload_path", &self.upload_path)
            .field("export_path", &self.export_path)
            .field("port", &self.port)
            .field("environment", &self.environment)
            .field("allowed_origins", &self.allowed_origins)
            .field("graphql_endpoint", &self.graphql_endpoint)
            .field("depth_limit", &self.depth_limit)
            .field("complexity_limit", &self.complexity_limit)
            .field("base_url", &self.base_url)
            .finish()
    }
}

impl AppConfig {
    pub fn from_env() -> Result<Self, KernelError> {
        dotenv().ok();

        let port = extract_env::<u16>("PORT")?;
        let max_db_connections = extract_env::<u32>("MAX_DB_CONNECTIONS")?;
        let body_limit_mb = extract_env::<usize>("BODY_LIMIT_MB")?;

        let export_path = extract_env("EXPORT_PATH").unwrap_or_else(|_| "/tmp".to_string());
        let upload_path = extract_env("UPLOAD_PATH").unwrap_or_else(|_| "/tmp".to_string());

        let environment: Environment = extract_env("ENVIRONMENT")?;

        let graphql_endpoint = env::var("GRAPHQL_ENDPOINT").unwrap_or_else(|_| "/orchard".into());
        let base_url = env::var("BASE_URL").unwrap_or_else(|_| format!("http://localhost:{port}"));

        let depth_limit = env::var("DEPTH_LIMIT")
            .ok()
            .map(|v| {
                v.parse::<usize>().map_err(|_| {
                    KernelError::EnvError("DEPTH_LIMIT must be a valid number".to_string())
                })
            })
            .transpose()?;

        let complexity_limit = env::var("COMPLEXITY_LIMIT")
            .ok()
            .map(|v| {
                v.parse::<usize>().map_err(|_| {
                    KernelError::EnvError("COMPLEXITY_LIMIT must be a valid number".to_string())
                })
            })
            .transpose()?;

        // Parse allowed origins (comma-separated)
        let raw_origins =
            extract_env::<String>("ALLOWED_ORIGINS").unwrap_or_else(|_| "*".to_string());
        let allowed_origin_strings: Vec<String> = if raw_origins == "*" {
            vec![]
        } else {
            raw_origins
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect()
        };
        let allowed_origins = if allowed_origin_strings.is_empty() {
            AllowOrigin::any()
        } else {
            let parsed = allowed_origin_strings
                .iter()
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<_>>();
            AllowOrigin::list(parsed)
        };

        let rate_limit_rps = env::var("RATE_LIMIT_RPS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(20);
        let rate_limit_burst = env::var("RATE_LIMIT_BURST")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(40);
        let rate_limit_rpm = env::var("RATE_LIMIT_RPM")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(300);
        let rate_limit_max_keys = env::var("RATE_LIMIT_MAX_KEYS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10_000);
        let timeout_secs = env::var("TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(25);

        let email_provider = parse_env("EMAIL_PROVIDER", default_email_provider())?;
        // `SMTP_ENCRYPTION` predates `SMTP_TLS_MODE`; keep reading it so existing
        // deployments do not silently fall back to STARTTLS on upgrade.
        let smtp_tls_mode = match env::var("SMTP_TLS_MODE") {
            Ok(_) => parse_env("SMTP_TLS_MODE", default_smtp_tls_mode())?,
            Err(_) => parse_env("SMTP_ENCRYPTION", default_smtp_tls_mode())?,
        };

        let smtp_host = env::var("SMTP_HOST").unwrap_or_default();
        let smtp_username = env::var("SMTP_AUTH_USERNAME").unwrap_or_default();
        let smtp_password = SecretString::from(env::var("SMTP_AUTH_PASSWORD").unwrap_or_default());

        let mailtrap_api_token =
            SecretString::from(env::var("MAILTRAP_API_TOKEN").unwrap_or_default());
        let mailtrap_base_url =
            env::var("MAILTRAP_BASE_URL").unwrap_or_else(|_| default_mailtrap_base_url());

        let redis_url = env::var("REDIS_CONNECTION_URL")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| KernelError::EnvError("REDIS_CONNECTION_URL".to_string()))?;

        if environment.is_production() && smtp_tls_mode == SmtpTlsMode::None {
            return Err(KernelError::EnvError(
                "SMTP_TLS_MODE=none is not allowed in production".to_string(),
            ));
        }

        if email_provider == EmailProvider::Smtp && smtp_host.trim().is_empty() {
            return Err(KernelError::EnvError(
                "SMTP_HOST is required when EMAIL_PROVIDER=smtp".to_string(),
            ));
        }

        if email_provider == EmailProvider::Mailtrap {
            use secrecy::ExposeSecret;

            if mailtrap_api_token.expose_secret().trim().is_empty() {
                return Err(KernelError::EnvError(
                    "MAILTRAP_API_TOKEN is required when EMAIL_PROVIDER=mailtrap".to_string(),
                ));
            }
        }

        Ok(Self {
            database_url: extract_env("DATABASE_URL")?,
            max_db_connections,
            body_limit_mb,
            upload_path,
            export_path,
            port,
            environment,
            allowed_origins,
            allowed_origin_strings,
            graphql_endpoint,
            depth_limit,
            complexity_limit,
            base_url,
            rate_limit_rps,
            rate_limit_burst,
            rate_limit_rpm,
            rate_limit_max_keys,
            timeout_secs,
            smtp_host,
            smtp_port: env::var("SMTP_PORT")
                .unwrap_or_else(|_| "587".into())
                .parse()
                .unwrap_or(587),
            smtp_username,
            smtp_password,
            smtp_tls_mode,
            email_provider,
            mailtrap_api_token,
            mailtrap_base_url,
            email_from_address: env::var("EMAIL_FROM_ADDRESS")
                .unwrap_or_else(|_| default_email_from_address()),
            email_from_name: env::var("EMAIL_FROM_NAME")
                .unwrap_or_else(|_| default_email_from_name()),
            email_transactions_from_address: env::var("EMAIL_TRANSACTIONS_FROM_ADDRESS")
                .unwrap_or_else(|_| default_email_transactions_from_address()),
            email_transactions_from_name: env::var("EMAIL_TRANSACTIONS_FROM_NAME")
                .unwrap_or_else(|_| default_email_transactions_from_name()),
            email_support_from_address: env::var("EMAIL_SUPPORT_FROM_ADDRESS")
                .unwrap_or_else(|_| default_email_support_from_address()),
            email_support_from_name: env::var("EMAIL_SUPPORT_FROM_NAME")
                .unwrap_or_else(|_| default_email_support_from_name()),
            redis_url: SecretString::from(redis_url),
        })
    }
}

use std::{env, str::FromStr};

use dotenv::dotenv;
use kernel::{error::KernelError, utils::extract_env};
use tower_http::cors::AllowOrigin;

use crate::errors::app_error::AppError;

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
    pub smtp_password: String,
    pub smtp_encryption: String,
}

#[derive(Debug, PartialEq, Eq)]
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

        let environment = extract_env("ENVIRONMENT")?;

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
        let raw_origins = extract_env::<String>("ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "*".to_string());
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
            smtp_host: env::var("SMTP_HOST").unwrap_or_default(),
            smtp_port: env::var("SMTP_PORT")
                .unwrap_or_else(|_| "587".into())
                .parse()
                .unwrap_or(587),
            smtp_username: env::var("SMTP_AUTH_USERNAME").unwrap_or_default(),
            smtp_password: env::var("SMTP_AUTH_PASSWORD").unwrap_or_default(),
            smtp_encryption: env::var("SMTP_ENCRYPTION").unwrap_or_else(|_| "starttls".into()),
        })
    }
}

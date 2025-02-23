use dotenv::dotenv;
use garde::Validate;
use once_cell::sync::OnceCell;
use std::env;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Validation error: {0}")]
    ValidationError(#[from] garde::Error),
    #[error("Environment error: {0}")]
    EnvError(#[from] env::VarError),

    #[error("Parse int error: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("Validation report: {0}")]
    ValidationReport(#[from] garde::Report),
}

#[derive(Validate, Debug, Clone)]
pub struct EnvConfig {
    #[garde(range(min = 1024, max = 65535))]
    pub port: u16,

    #[garde(length(min = 1))]
    pub host: String,

    #[garde(length(min = 1))]
    pub app_env: String,

    #[garde(length(min = 1))]
    pub braze_api_key: String,

    #[garde(length(min = 1))]
    pub database_url: String,
}

// implement default trait
impl Default for EnvConfig {
    fn default() -> Self {
        Self {
            port: 8088,
            host: "localhost".to_owned(),
            app_env: "development".to_owned(),
            braze_api_key: "".to_owned(), // empty string
            database_url: "".to_owned(),
        }
    }
}

// Singleton instance
static INSTANCE: OnceCell<EnvConfig> = OnceCell::new();

impl EnvConfig {
    pub fn init() -> Result<(), ConfigError> {
        let config = Self::from_env()?;
        INSTANCE.set(config).unwrap();
        Ok(())
    }

    pub fn get_instance() -> &'static EnvConfig {
        INSTANCE.get().expect("EnvConfig not initialized")
    }

    fn get_env_or_default<T: std::str::FromStr>(key: &str, default: T) -> T {
        env::var(key)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    fn from_env() -> Result<Self, ConfigError> {
        let app_env = env::var("APP_ENV").unwrap_or("development".to_string());
        if app_env == "development" {
            dotenv().ok();
        }

        let defaults = Self::default();

        let config = EnvConfig {
            port: Self::get_env_or_default("APP_PORT", defaults.port),
            host: Self::get_env_or_default("APP_HOST", defaults.host),
            app_env,
            braze_api_key: Self::get_env_or_default("APP_BRAZE_API_KEY", defaults.braze_api_key),
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| defaults.database_url),
        };

        config.validate(&())?;
        Ok(config)
    }
}

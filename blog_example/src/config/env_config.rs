use dotenv::dotenv;
use once_cell::sync::OnceCell;
use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Environment variable not found: {0}")]
    MissingEnv(String),
    #[error("Failed to parse environment variable: {0}")]
    ParseError(String),
}

#[derive(Debug, Clone)]
pub struct EnvConfig {
    pub port: u16,
    pub host: String,
    pub app_env: String,
}

static INSTANCE: OnceCell<EnvConfig> = OnceCell::new();

impl EnvConfig {
    pub fn init() -> Result<(), ConfigError> {
        let app_env = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

        // Load .env file only in local environment
        if app_env == "local" {
            dotenv().ok();
        }

        let config = EnvConfig {
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .map_err(|_| ConfigError::ParseError("PORT".to_string()))?,
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            app_env,
        };

        INSTANCE.set(config).unwrap();
        Ok(())
    }

    pub fn get_instance() -> &'static EnvConfig {
        INSTANCE.get().expect("EnvConfig not initialized")
    }
}

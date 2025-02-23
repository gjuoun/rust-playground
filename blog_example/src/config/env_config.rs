use config::{Config, Environment, File, FileFormat};
use dotenv::dotenv;
use garde::Validate;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Validation error: {0}")]
    ValidationError(#[from] garde::Error),
    #[error("Config error: {0}")]
    ConfigError(#[from] config::ConfigError),
    #[error("Validation report: {0}")]
    ValidationReport(#[from] garde::Report),
}

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct EnvConfig {
    #[garde(range(min = 1024, max = 65535))]
    pub port: u16,

    #[garde(length(min = 1))]
    pub host: String,

    #[garde(length(min = 1))]
    pub app_env: String,

    #[garde(length(min = 1))]
    pub braze_api_key: String,
}

impl Default for EnvConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            host: "localhost".to_string(),
            app_env: "local".to_string(),
            braze_api_key: String::new(),
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

    fn from_env() -> Result<Self, ConfigError> {
        let defaults: Self = Self::default();

        // Set up configuration with defaults
        let mut builder = Config::builder()
            .set_default("port", defaults.port)?
            .set_default("host", defaults.host)?
            .set_default("app_env", defaults.app_env)?
            .set_default("braze_api_key", defaults.braze_api_key)?;

        // Load .env file only in local environment
        let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
        if app_env == "local" {
            dotenv().ok();
            // Try to load .env file if it exists, ignore if it doesn't
            let file_source = File::new(".env", FileFormat::Ini).required(false);
            builder = builder.add_source(file_source);
        }

        // Add environment variables with prefix support and case sensitivity
        builder = builder.add_source(
            Environment::default()
                .separator("_")
                .try_parsing(true)
                .with_list_parse_key("list_items"),
        );

        // Build the config
        let config = builder.build()?;

        // Deserialize into EnvConfig
        let env_config: EnvConfig = config.try_deserialize()?;

        // Validate using garde
        env_config.validate(&())?;

        Ok(env_config)
    }
}

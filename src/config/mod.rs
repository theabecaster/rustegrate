use dotenvy::dotenv;
use serde::Deserialize;
use std::env;

/// Application configuration settings
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// Host address to bind the server to
    pub host: String,
    
    /// Port to bind the server to
    pub port: u16,
    
    /// Log level (e.g., "info", "debug", "trace")
    pub log_level: String,
    
    /// Optional database URL for persistent storage
    pub database_url: Option<String>,
}

impl AppConfig {
    /// Load configuration from environment variables and .env file
    pub fn load() -> Result<Self, config::ConfigError> {
        // Load .env file if it exists
        let _ = dotenv();
        
        // Default configuration
        let default_config = Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            log_level: "info".to_string(),
            database_url: None,
        };
        
        // Load configuration from environment variables
        let host = env::var("HOST").unwrap_or(default_config.host);
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(default_config.port);
        let log_level = env::var("LOG_LEVEL").unwrap_or(default_config.log_level);
        let database_url = env::var("DATABASE_URL").ok();
        
        Ok(Self {
            host,
            port,
            log_level,
            database_url,
        })
    }
} 
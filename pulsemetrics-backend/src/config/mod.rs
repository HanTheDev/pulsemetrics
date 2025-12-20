use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub app: AppConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn socket_addr(&self) -> anyhow::Result<SocketAddr> {
        let addr = format!("{}:{}", self.host, self.port);
        addr.parse()
            .map_err(|e| anyhow::anyhow!("Invalid socket address: {}", e))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_seconds: u64,
    pub idle_timeout_seconds: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub environment: Environment,
    pub api_key: String,
    pub max_batch_size: usize,
    pub buffer_flush_interval_ms: u64,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Production,
    Test,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let config = Config {
            server: ServerConfig {
                host: std::env::var("SERVER_HOST")
                    .unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: std::env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8000".to_string())
                    .parse()
                    .map_err(|e| anyhow::anyhow!("Invalid SERVER_PORT: {}", e))?,
            },
            database: DatabaseConfig {
                url: std::env::var("DATABASE_URL")
                    .map_err(|_| anyhow::anyhow!("DATABASE_URL must be set"))?,
                max_connections: std::env::var("DB_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "20".to_string())
                    .parse()?,
                min_connections: std::env::var("DB_MIN_CONNECTIONS")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()?,
                connect_timeout_seconds: std::env::var("DB_CONNECT_TIMEOUT")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()?,
                idle_timeout_seconds: std::env::var("DB_IDLE_TIMEOUT")
                    .unwrap_or_else(|_| "600".to_string())
                    .parse()?,
            },
            app: AppConfig {
                environment: std::env::var("ENVIRONMENT")
                    .unwrap_or_else(|_| "development".to_string())
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid ENVIRONMENT"))?,
                api_key: std::env::var("API_KEY")
                    .unwrap_or_else(|_| "dev-api-key-change-in-production".to_string()),
                max_batch_size: std::env::var("MAX_BATCH_SIZE")
                    .unwrap_or_else(|_| "1000".to_string())
                    .parse()?,
                buffer_flush_interval_ms: std::env::var("BUFFER_FLUSH_INTERVAL_MS")
                    .unwrap_or_else(|_| "100".to_string())
                    .parse()?,
            },
        };

        Ok(config)
    }
}

impl std::str::FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "development" => Ok(Environment::Development),
            "production" => Ok(Environment::Production),
            "test" => Ok(Environment::Test),
            _ => Err(format!("Unknown environment: {}", s)),
        }
    }
}
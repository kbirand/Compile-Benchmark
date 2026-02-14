use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct AppConfig {
    #[builder(default = DatabaseConfig::default())]
    pub database: DatabaseConfig,
    #[builder(default = ServerConfig::default())]
    pub server: ServerConfig,
    #[builder(default = AuthConfig::default())]
    pub auth: AuthConfig,
    #[builder(default = CacheConfig::default())]
    pub cache: CacheConfig,
    #[builder(default = ExternalServices::default())]
    pub external: ExternalServices,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DatabaseConfig {
    #[builder(default = "sqlite::memory:".to_string())]
    pub url: String,
    #[builder(default = 10)]
    pub max_connections: u32,
    #[builder(default = 30)]
    pub timeout_seconds: u64,
    #[builder(default = true)]
    pub enable_logging: bool,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ServerConfig {
    #[builder(default = "127.0.0.1".to_string())]
    pub host: String,
    #[builder(default = 8080)]
    pub port: u16,
    #[builder(default = 100)]
    pub max_body_size_mb: usize,
    #[builder(default = true)]
    pub enable_compression: bool,
    #[builder(default = true)]
    pub enable_cors: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct AuthConfig {
    #[builder(default = "super-secret-jwt-key-change-in-production".to_string())]
    pub jwt_secret: String,
    #[builder(default = 3600)]
    pub token_expiry_seconds: i64,
    #[builder(default = 86400)]
    pub refresh_token_expiry_seconds: i64,
    #[builder(default = 10)]
    pub bcrypt_cost: u32,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CacheConfig {
    #[builder(default = 10000)]
    pub max_capacity: u64,
    #[builder(default = 300)]
    pub ttl_seconds: u64,
    #[builder(default = true)]
    pub enable_metrics: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ExternalServices {
    #[builder(default = "https://api.example.com".to_string())]
    pub api_base_url: String,
    #[builder(default = "".to_string())]
    pub api_key: String,
    #[builder(default = 30)]
    pub timeout_seconds: u64,
    #[builder(default = 3)]
    pub retry_attempts: u32,
}

impl Default for ExternalServices {
    fn default() -> Self {
        Self::builder().build()
    }
}

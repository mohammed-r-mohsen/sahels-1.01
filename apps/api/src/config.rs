use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub port: u16,
    pub host: String,
    pub jwt_secret: String,
    pub encryption_key: String,
    pub rate_limit_requests_per_minute: usize,
    pub rate_limit_burst: usize,
}

impl AppConfig {
    pub fn load() -> Self {
        Self {
            port: env::var("API_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            host: env::var("API_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "dev-secret-change-me".to_string()),
            encryption_key: env::var("ENCRYPTION_KEY")
                .unwrap_or_else(|_| "base64:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".to_string()),
            rate_limit_requests_per_minute: env::var("RATE_LIMIT_REQUESTS_PER_MINUTE")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .unwrap_or(60),
            rate_limit_burst: env::var("RATE_LIMIT_BURST")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
        }
    }
}

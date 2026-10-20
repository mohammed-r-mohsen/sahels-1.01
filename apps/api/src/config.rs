use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub port: u16,
    pub host: String,
    pub jwt_secret: String,
    pub encryption_key: String,
    pub rate_limit_requests_per_minute: usize,
    pub rate_limit_burst: usize,
    pub clinic_api_token: String,
    pub clinic_payment_token: String,
    pub clinic_whatsapp_token: String,
    pub clinic_instagram_token: String,
    pub clinic_otp_secret: String,
    pub clinic_otp_ttl_seconds: u64,
    pub backup_dir: String,
    pub backup_retention_days: u64,
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
            clinic_api_token: env::var("CLINIC_API_TOKEN")
                .unwrap_or_else(|_| "replace-clinic-api-token".to_string()),
            clinic_payment_token: env::var("CLINIC_PAYMENT_TOKEN")
                .unwrap_or_else(|_| "replace-clinic-payment-token".to_string()),
            clinic_whatsapp_token: env::var("CLINIC_WHATSAPP_TOKEN")
                .unwrap_or_else(|_| "replace-whatsapp-token".to_string()),
            clinic_instagram_token: env::var("CLINIC_INSTAGRAM_TOKEN")
                .unwrap_or_else(|_| "replace-instagram-token".to_string()),
            clinic_otp_secret: env::var("CLINIC_OTP_SECRET")
                .unwrap_or_else(|_| "replace-otp-secret".to_string()),
            clinic_otp_ttl_seconds: env::var("CLINIC_OTP_TTL_SECONDS")
                .unwrap_or_else(|_| "300".to_string())
                .parse()
                .unwrap_or(300),
            backup_dir: env::var("BACKUP_DIR").unwrap_or_else(|_| "/opt/sahels/backups".to_string()),
            backup_retention_days: env::var("BACKUP_RETENTION_DAYS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
        }
    }
}

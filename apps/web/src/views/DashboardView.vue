use axum::{
    extract::FromRef,
    http::{HeaderValue, StatusCode},
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod middleware;
mod models;
mod routes;
mod security;

use config::AppConfig;
use middleware::rate_limit::RateLimiter;
use routes::{appointments::*, auth::*, clinics::*, health::*, reminders::*};
use security::AppSecurity;

#[derive(Clone)]
pub struct AppState {
    pub security: std::sync::Arc<AppSecurity>,
    pub rate_limiter: std::sync::Arc<RateLimiter>,
    pub config: AppConfig,
    pub pool: PgPool,
}

impl FromRef<AppState> for std::sync::Arc<AppSecurity> {
    fn from_ref(state: &AppState) -> Self {
        state.security.clone()
    }
}

impl FromRef<AppState> for std::sync::Arc<RateLimiter> {
    fn from_ref(state: &AppState) -> Self {
        state.rate_limiter.clone()
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::load();
    let security = std::sync::Arc::new(AppSecurity::new(
        config.jwt_secret.clone(),
        config.encryption_key.clone(),
    ));

    let rate_limiter = std::sync::Arc::new(RateLimiter::new(
        config.rate_limit_requests_per_minute,
        config.rate_limit_burst,
    ));

    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("failed to connect to PostgreSQL");

    let state = AppState {
        security,
        rate_limiter,
        config: config.clone(),
        pool,
    };

    let allowed_origins = [
        "http://localhost:5173",
        "http://127.0.0.1:5173",
    ];

    let cors = CorsLayer::new()
        .allow_origin(allowed_origins.map(|origin| origin.parse::<HeaderValue>().unwrap()))
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::PATCH,
        ])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
        ]);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/auth/login", post(login_handler))
        .route("/api/v1/auth/register", post(register_handler))
        .route("/api/v1/auth/otp/request", post(otp_request_handler))
        .route("/api/v1/auth/otp/verify", post(otp_verify_handler))
        .route("/api/v1/appointments", get(list_appointments))
        .route("/api/v1/appointments", post(create_appointment))
        .route("/api/v1/reminders/queue", post(queue_reminder))
        .route("/api/v1/clinic/summary", get(clinic_summary))
        .route("/api/v1/clinic/doctors", get(list_doctors))
        .route("/api/v1/clinic/doctors", post(upsert_doctor))
        .route("/api/v1/clinic/payment-methods", get(list_payment_methods))
        .route("/api/v1/clinic/payment-methods", post(upsert_payment_method))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Sahels API listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind to port");

    axum::serve(listener, app).await.expect("server failure");
}

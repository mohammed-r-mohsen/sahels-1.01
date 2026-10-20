use axum::{
    http::{HeaderValue, StatusCode},
    routing::{get, post},
    Json, Router,
};
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod middleware;
mod routes;
mod security;

use config::AppConfig;
use middleware::rate_limit::RateLimiter;
use routes::{auth::*, health::*};
use security::AppSecurity;

#[derive(Clone)]
pub struct AppState {
    pub security: Arc<AppSecurity>,
    pub rate_limiter: Arc<RateLimiter>,
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

    let security = Arc::new(AppSecurity::new(
        config.jwt_secret.clone(),
        config.encryption_key.clone(),
    ));

    let rate_limiter = Arc::new(RateLimiter::new(
        config.rate_limit_requests_per_minute,
        config.rate_limit_burst,
    ));

    let state = AppState {
        security,
        rate_limiter,
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::PUT, axum::http::Method::DELETE])
        .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::AUTHORIZATION]);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/auth/login", post(login_handler))
        .route("/api/v1/auth/register", post(register_handler))
        .route("/api/v1/appointments", get(appointments_index))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Sahels API listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind to port");

    axum::serve(listener, app).await.expect("server failure");
}

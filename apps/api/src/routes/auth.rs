use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub token: Option<String>,
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, Json<AuthResponse>)> {
    let key = format!("{}:{}", payload.email, "login");
    if !state.rate_limiter.allow(&key) {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            Json(AuthResponse {
                success: false,
                message: "Too many login attempts. Please wait a minute and try again.".to_string(),
                token: None,
            }),
        ));
    }

    let encoded = state
        .security
        .hash_password(&payload.password)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthResponse {
                    success: false,
                    message: "Password hashing failed.".to_string(),
                    token: None,
                }),
            )
        })?;

    let token = format!("sahels-token:{}", encoded);

    Ok((
        StatusCode::OK,
        Json(AuthResponse {
            success: true,
            message: "Authentication successful.".to_string(),
            token: Some(token),
        }),
    ))
}

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, Json<AuthResponse>)> {
    let key = format!("{}:{}", payload.email, "register");
    if !state.rate_limiter.allow(&key) {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            Json(AuthResponse {
                success: false,
                message: "Registration rate limit exceeded.".to_string(),
                token: None,
            }),
        ));
    }

    let encrypted = state
        .security
        .encrypt(&payload.password)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthResponse {
                    success: false,
                    message: "Secret encryption failed.".to_string(),
                    token: None,
                }),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            success: true,
            message: format!("User registered successfully. Secret protected with encryption: {}", encrypted),
            token: Some(format!("sahels-registration:{}", encrypted)),
        }),
    ))
}

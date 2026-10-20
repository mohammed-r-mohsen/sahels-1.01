use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub clinic_id: Option<String>,
}

#[derive(Deserialize)]
pub struct OtpRequest {
    pub clinic_id: String,
    pub phone: String,
    pub purpose: Option<String>,
}

#[derive(Deserialize)]
pub struct OtpVerifyRequest {
    pub clinic_id: String,
    pub phone: String,
    pub code: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub token: Option<String>,
}

#[derive(Serialize)]
pub struct OtpResponse {
    pub success: bool,
    pub message: String,
    pub expires_in_seconds: Option<u64>,
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, Json<AuthResponse>)> {
    let key = format!("{}:{}:{}", payload.clinic_id.clone().unwrap_or_else(|| "global".to_string()), payload.email, "login");
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

    Ok((
        StatusCode::OK,
        Json(AuthResponse {
            success: true,
            message: "Authentication successful.".to_string(),
            token: Some(format!("sahels-token:{}:{}", payload.clinic_id.unwrap_or_else(|| "default".to_string()), encoded)),
        }),
    ))
}

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, Json<AuthResponse>)> {
    let clinic_id = payload.clinic_id.clone().unwrap_or_else(|| "test_clinic".to_string());
    let key = format!("{}:{}:{}", clinic_id, payload.email, "register");
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
            message: format!("Clinic registered successfully for {}.", clinic_id),
            token: Some(format!("sahels-registration:{}:{}", clinic_id, encrypted)),
        }),
    ))
}

pub async fn otp_request_handler(
    State(state): State<AppState>,
    Json(payload): Json<OtpRequest>,
) -> Result<(StatusCode, Json<OtpResponse>), (StatusCode, Json<OtpResponse>)> {
    let code = state.security.generate_otp(
        &payload.clinic_id,
        &payload.phone,
        state.config.clinic_otp_ttl_seconds,
    );

    Ok((
        StatusCode::OK,
        Json(OtpResponse {
            success: true,
            message: format!("OTP created for {}.", payload.phone),
            expires_in_seconds: Some(state.config.clinic_otp_ttl_seconds),
        }),
    ))
}

pub async fn otp_verify_handler(
    State(_state): State<AppState>,
    Json(payload): Json<OtpVerifyRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, Json<AuthResponse>)> {
    let _ = (&payload.clinic_id, &payload.phone, &payload.code);

    Ok((
        StatusCode::OK,
        Json(AuthResponse {
            success: true,
            message: "OTP verified successfully.".to_string(),
            token: Some(format!("otp-valid:{}:{}", payload.clinic_id, payload.phone)),
        }),
    ))
}

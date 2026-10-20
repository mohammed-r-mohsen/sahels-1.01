use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::AppState;

#[derive(Clone, Debug)]
pub struct AuthContext {
    pub user_id: String,
    pub clinic_id: String,
    pub email: String,
    pub role: String,
}

pub async fn require_tenant(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let value = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = state
        .security
        .verify_jwt(value)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let request_clinic = request
        .headers()
        .get("x-clinic-id")
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    if let Some(expected) = request_clinic {
        if expected != claims.clinic_id {
            return Err(StatusCode::FORBIDDEN);
        }
    }

    request.extensions_mut().insert(AuthContext {
        user_id: claims.sub,
        clinic_id: claims.clinic_id.clone(),
        email: claims.email,
        role: claims.role,
    });

    Ok(next.run(request).await)
}

use axum::{extract::State, Json};
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
    pub version: &'static str,
}

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "sahels-api",
        version: "1.0.0",
    })
}

pub async fn appointments_index(State(_state): State<AppState>) -> Json<Vec<String>> {
    Json(vec![
        "appointment-1".to_string(),
        "appointment-2".to_string(),
        "appointment-3".to_string(),
    ])
}

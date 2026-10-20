use axum::{extract::State, Json};
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
    pub version: &'static str,
}

#[derive(Serialize)]
pub struct AppointmentItem {
    pub id: usize,
    pub patient_name: String,
    pub patient_phone: String,
    pub appointment_date: String,
    pub appointment_time: String,
    pub status: &'static str,
}

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "sahels-api",
        version: "1.0.0",
    })
}

pub async fn appointments_index(State(_state): State<AppState>) -> Json<Vec<AppointmentItem>> {
    Json(vec![
        AppointmentItem {
            id: 1,
            patient_name: "أحمد السلمان".to_string(),
            patient_phone: "+966500000001".to_string(),
            appointment_date: "2026-10-22".to_string(),
            appointment_time: "09:30".to_string(),
            status: "confirmed",
        },
        AppointmentItem {
            id: 2,
            patient_name: "سارة القحطاني".to_string(),
            patient_phone: "+966500000002".to_string(),
            appointment_date: "2026-10-22".to_string(),
            appointment_time: "11:00".to_string(),
            status: "pending",
        },
    ])
}

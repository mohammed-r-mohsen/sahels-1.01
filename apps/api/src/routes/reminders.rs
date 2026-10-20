use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{models::Appointment, AppState};

#[derive(Deserialize)]
pub struct CreateAppointmentRequest {
    pub clinic_id: String,
    pub doctor_id: Option<i32>,
    pub patient_name: String,
    pub patient_phone: String,
    pub patient_email: Option<String>,
    pub appointment_date: String,
    pub appointment_time: String,
    pub duration_minutes: Option<i32>,
    pub service: Option<String>,
    pub source: Option<String>,
}

#[derive(Serialize)]
pub struct AppointmentResponse {
    pub id: i32,
    pub clinic_id: String,
    pub patient_name: String,
    pub patient_phone: String,
    pub appointment_date: String,
    pub appointment_time: String,
    pub status: String,
}

pub async fn list_appointments(
    State(state): State<AppState>,
) -> Result<Json<Vec<AppointmentResponse>>, (StatusCode, Json<String>)> {
    let rows = sqlx::query_as::<_, Appointment>(
        r#"
        SELECT id, clinic_id, doctor_id, patient_name, patient_phone, patient_email,
               appointment_date, appointment_time, duration_minutes, service, source,
               status, reminder_sent, reminder_count, risk_score, consent_given,
               deposit_paid, deposit_amount_sar
        FROM appointments
        ORDER BY appointment_date, appointment_time
        LIMIT 25
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json("Database query failed".to_string())))?;

    let items = rows
        .into_iter()
        .map(|row| AppointmentResponse {
            id: row.id,
            clinic_id: row.clinic_id,
            patient_name: row.patient_name,
            patient_phone: row.patient_phone,
            appointment_date: row.appointment_date.to_string(),
            appointment_time: row.appointment_time.to_string(),
            status: row.status,
        })
        .collect();

    Ok(Json(items))
}

pub async fn create_appointment(
    State(state): State<AppState>,
    Json(payload): Json<CreateAppointmentRequest>,
) -> Result<(StatusCode, Json<AppointmentResponse>), (StatusCode, Json<String>)> {
    let appointment_date = chrono::NaiveDate::parse_from_str(&payload.appointment_date, "%Y-%m-%d")
        .map_err(|_| (StatusCode::BAD_REQUEST, Json("Invalid appointment_date format".to_string())))?;
    let appointment_time = chrono::NaiveTime::parse_from_str(&payload.appointment_time, "%H:%M")
        .map_err(|_| (StatusCode::BAD_REQUEST, Json("Invalid appointment_time format".to_string())))?;

    let source = payload.source.clone().unwrap_or_else(|| "whatsapp".to_string());
    let record = sqlx::query_as::<_, Appointment>(
        r#"
        INSERT INTO appointments (
            clinic_id, doctor_id, patient_name, patient_phone, patient_email,
            appointment_date, appointment_time, duration_minutes, service, source, status
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 'pending')
        RETURNING id, clinic_id, doctor_id, patient_name, patient_phone, patient_email,
                  appointment_date, appointment_time, duration_minutes, service, source,
                  status, reminder_sent, reminder_count, risk_score, consent_given,
                  deposit_paid, deposit_amount_sar
        "#,
    )
    .bind(&payload.clinic_id)
    .bind(payload.doctor_id)
    .bind(&payload.patient_name)
    .bind(&payload.patient_phone)
    .bind(&payload.patient_email)
    .bind(appointment_date)
    .bind(appointment_time)
    .bind(payload.duration_minutes)
    .bind(&payload.service)
    .bind(source)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to create appointment".to_string())))?;

    Ok((
        StatusCode::CREATED,
        Json(AppointmentResponse {
            id: record.id,
            clinic_id: record.clinic_id,
            patient_name: record.patient_name,
            patient_phone: record.patient_phone,
            appointment_date: record.appointment_date.to_string(),
            appointment_time: record.appointment_time.to_string(),
            status: record.status,
        }),
    ))
}

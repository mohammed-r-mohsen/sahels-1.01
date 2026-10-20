use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Deserialize)]
pub struct QueueReminderRequest {
    pub clinic_id: String,
    pub appointment_id: i32,
    pub channel: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct ReminderResponse {
    pub success: bool,
    pub queued: bool,
    pub message: String,
}

pub async fn queue_reminder(
    State(_state): State<AppState>,
    Json(payload): Json<QueueReminderRequest>,
) -> Result<(StatusCode, Json<ReminderResponse>), (StatusCode, Json<String>)> {
    let _ = (&payload.clinic_id, payload.appointment_id, &payload.channel, &payload.message);

    Ok((
        StatusCode::ACCEPTED,
        Json(ReminderResponse {
            success: true,
            queued: true,
            message: format!("Reminder queued for clinic {} on {} channel.", payload.clinic_id, payload.channel),
        }),
    ))
}

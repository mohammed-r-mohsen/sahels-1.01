use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Clinic {
    pub id: String,
    pub name: String,
    pub legal_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub status: String,
    pub timezone: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Doctor {
    pub id: i32,
    pub clinic_id: String,
    pub full_name: String,
    pub specialty: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Appointment {
    pub id: i32,
    pub clinic_id: String,
    pub doctor_id: Option<i32>,
    pub patient_name: String,
    pub patient_phone: String,
    pub patient_email: Option<String>,
    pub appointment_date: chrono::NaiveDate,
    pub appointment_time: chrono::NaiveTime,
    pub duration_minutes: Option<i32>,
    pub service: Option<String>,
    pub source: String,
    pub status: String,
    pub reminder_sent: Option<bool>,
    pub reminder_count: Option<i32>,
    pub risk_score: Option<Decimal>,
    pub consent_given: Option<bool>,
    pub deposit_paid: Option<bool>,
    pub deposit_amount_sar: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ReminderJob {
    pub id: i32,
    pub clinic_id: String,
    pub appointment_id: Option<i32>,
    pub channel: String,
    pub status: String,
    pub payload: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentTokenConfig {
    pub payment_token: String,
    pub whatsapp_token: String,
    pub instagram_token: String,
    pub api_token: String,
    pub otp_secret: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenPayload {
    pub clinic_id: String,
    pub email: String,
}

#[derive(Debug, Clone, Copy)]
pub struct Decimal;

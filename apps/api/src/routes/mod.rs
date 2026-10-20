use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{models::{ClinicSummary, DoctorAvailability, PaymentMethodConfig}, AppState};

#[derive(Deserialize)]
pub struct ClinicFilter {
    pub clinic_id: Option<String>,
}

#[derive(Deserialize)]
pub struct DoctorUpsertRequest {
    pub clinic_id: String,
    pub full_name: String,
    pub specialty: Option<String>,
    pub is_active: Option<bool>,
    pub is_available: Option<bool>,
    pub max_patients_per_slot: Option<i32>,
    pub work_days: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct DoctorResponse {
    pub id: i32,
    pub clinic_id: String,
    pub full_name: String,
    pub specialty: String,
    pub is_active: bool,
    pub is_available: bool,
    pub max_patients_per_slot: i32,
    pub work_days: Vec<String>,
}

#[derive(Deserialize)]
pub struct PaymentMethodUpsertRequest {
    pub clinic_id: String,
    pub provider: String,
    pub method_name: String,
    pub merchant_id: Option<String>,
    pub is_active: Option<bool>,
    pub deposit_required: Option<bool>,
    pub deposit_amount_sar: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Serialize)]
pub struct PaymentMethodResponse {
    pub id: i32,
    pub clinic_id: String,
    pub provider: String,
    pub method_name: String,
    pub merchant_id: Option<String>,
    pub is_active: bool,
    pub deposit_required: bool,
    pub deposit_amount_sar: f64,
    pub currency: String,
}

pub async fn clinic_summary(
    State(_state): State<AppState>,
    Query(params): Query<ClinicFilter>,
) -> Result<Json<ClinicSummary>, (StatusCode, Json<String>)> {
    let clinic_id = params.clinic_id.clone().unwrap_or_else(|| "test_clinic".to_string());

    Ok(Json(ClinicSummary {
        id: clinic_id,
        name: "عيادة النخيل".to_string(),
        city: "الرياض".to_string(),
        status: "online".to_string(),
        doctor_count: 6,
        available_doctors: 4,
        deposit_required: true,
        deposit_amount_sar: 50.0,
    }))
}

pub async fn list_doctors(
    State(_state): State<AppState>,
    Query(params): Query<ClinicFilter>,
) -> Result<Json<Vec<DoctorResponse>>, (StatusCode, Json<String>)> {
    let clinic_id = params.clinic_id.clone().unwrap_or_else(|| "test_clinic".to_string());

    let doctors = vec![
        DoctorResponse {
            id: 1,
            clinic_id: clinic_id.clone(),
            full_name: "د. علي الحربي".to_string(),
            specialty: "جلدية".to_string(),
            is_active: true,
            is_available: true,
            max_patients_per_slot: 8,
            work_days: vec!["الاثنين".to_string(), "الثلاثاء".to_string(), "الأربعاء".to_string()],
        },
        DoctorResponse {
            id: 2,
            clinic_id: clinic_id.clone(),
            full_name: "د. مها الزهراني".to_string(),
            specialty: "الأسنان".to_string(),
            is_active: true,
            is_available: true,
            max_patients_per_slot: 6,
            work_days: vec!["الأحد".to_string(), "الثلاثاء".to_string(), "الخميس".to_string()],
        },
        DoctorResponse {
            id: 3,
            clinic_id: clinic_id.clone(),
            full_name: "د. عبدالله القحطاني".to_string(),
            specialty: "مراجعة عامة".to_string(),
            is_active: true,
            is_available: false,
            max_patients_per_slot: 5,
            work_days: vec!["الأحد".to_string(), "السبت".to_string()],
        },
    ];

    Ok(Json(doctors))
}

pub async fn upsert_doctor(
    State(_state): State<AppState>,
    Json(payload): Json<DoctorUpsertRequest>,
) -> Result<(StatusCode, Json<DoctorResponse>), (StatusCode, Json<String>)> {
    let doctor = DoctorResponse {
        id: 99,
        clinic_id: payload.clinic_id.clone(),
        full_name: payload.full_name.clone(),
        specialty: payload.specialty.clone().unwrap_or_else(|| "عام".to_string()),
        is_active: payload.is_active.unwrap_or(true),
        is_available: payload.is_available.unwrap_or(true),
        max_patients_per_slot: payload.max_patients_per_slot.unwrap_or(5),
        work_days: payload.work_days.unwrap_or_else(|| vec!["الاثنين".to_string()]),
    };

    Ok((StatusCode::OK, Json(doctor)))
}

pub async fn list_payment_methods(
    State(_state): State<AppState>,
    Query(params): Query<ClinicFilter>,
) -> Result<Json<Vec<PaymentMethodResponse>>, (StatusCode, Json<String>)> {
    let clinic_id = params.clinic_id.clone().unwrap_or_else(|| "test_clinic".to_string());

    let methods = vec![
        PaymentMethodResponse {
            id: 1,
            clinic_id: clinic_id.clone(),
            provider: "moyasar".to_string(),
            method_name: "بوابة الدفع - Moyasar".to_string(),
            merchant_id: Some("merchant_001".to_string()),
            is_active: true,
            deposit_required: true,
            deposit_amount_sar: 50.0,
            currency: "SAR".to_string(),
        },
        PaymentMethodResponse {
            id: 2,
            clinic_id: clinic_id.clone(),
            provider: "tabby".to_string(),
            method_name: "تقسيط - Tabby".to_string(),
            merchant_id: Some("tabby_001".to_string()),
            is_active: true,
            deposit_required: false,
            deposit_amount_sar: 0.0,
            currency: "SAR".to_string(),
        },
    ];

    Ok(Json(methods))
}

pub async fn upsert_payment_method(
    State(_state): State<AppState>,
    Json(payload): Json<PaymentMethodUpsertRequest>,
) -> Result<(StatusCode, Json<PaymentMethodResponse>), (StatusCode, Json<String>)> {
    let method = PaymentMethodResponse {
        id: 12,
        clinic_id: payload.clinic_id.clone(),
        provider: payload.provider.clone(),
        method_name: payload.method_name.clone(),
        merchant_id: payload.merchant_id.clone(),
        is_active: payload.is_active.unwrap_or(true),
        deposit_required: payload.deposit_required.unwrap_or(true),
        deposit_amount_sar: payload.deposit_amount_sar.unwrap_or(50.0),
        currency: payload.currency.unwrap_or_else(|| "SAR".to_string()),
    };

    Ok((StatusCode::OK, Json(method)))
}

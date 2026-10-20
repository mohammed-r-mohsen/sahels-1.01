use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicSummary {
    pub id: String,
    pub name: String,
    pub city: String,
    pub status: String,
    pub doctor_count: usize,
    pub available_doctors: usize,
    pub deposit_required: bool,
    pub deposit_amount_sar: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctorAvailability {
    pub id: i32,
    pub clinic_id: String,
    pub full_name: String,
    pub specialty: String,
    pub is_active: bool,
    pub is_available: bool,
    pub max_patients_per_slot: i32,
    pub work_days: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodConfig {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicSettings {
    pub clinic_id: String,
    pub reminder_before_minutes: i32,
    pub reminder_medium: String,
    pub deposit_required: bool,
    pub deposit_amount_sar: f64,
    pub no_show_threshold: f64,
    pub backup_retention_days: i32,
}

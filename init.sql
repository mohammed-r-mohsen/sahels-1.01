CREATE TABLE IF NOT EXISTS clinics (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  legal_name TEXT,
  phone TEXT,
  email TEXT,
  city TEXT,
  region TEXT,
  status TEXT DEFAULT 'offline' CHECK (status IN ('offline','online','paused','trial')),
  timezone TEXT DEFAULT 'Asia/Riyadh',
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS clinic_tokens (
  id SERIAL PRIMARY KEY,
  clinic_id TEXT NOT NULL REFERENCES clinics(id) ON DELETE CASCADE,
  payment_token TEXT,
  whatsapp_token TEXT,
  instagram_token TEXT,
  api_token TEXT,
  otp_secret TEXT,
  otp_enabled BOOLEAN DEFAULT TRUE,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW(),
  UNIQUE(clinic_id)
);

CREATE TABLE IF NOT EXISTS clinic_settings (
  id SERIAL PRIMARY KEY,
  clinic_id TEXT NOT NULL REFERENCES clinics(id) ON DELETE CASCADE,
  reminder_before_minutes INT DEFAULT 180,
  reminder_medium TEXT DEFAULT 'whatsapp' CHECK (reminder_medium IN ('whatsapp','sms','email')),
  deposit_required BOOLEAN DEFAULT FALSE,
  deposit_amount_sar NUMERIC(10,2) DEFAULT 50,
  no_show_threshold NUMERIC(3,2) DEFAULT 0.10,
  working_hours JSONB DEFAULT '{"saturday_to_thursday": "09:00-17:00", "friday": "closed"}'::jsonb,
  consent_policy TEXT DEFAULT 'pdpl_v1',
  backup_retention_days INT DEFAULT 30,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW(),
  UNIQUE(clinic_id)
);

CREATE TABLE IF NOT EXISTS doctors (
  id SERIAL PRIMARY KEY,
  clinic_id TEXT NOT NULL REFERENCES clinics(id) ON DELETE CASCADE,
  full_name TEXT NOT NULL,
  specialty TEXT,
  phone TEXT,
  email TEXT,
  is_active BOOLEAN DEFAULT TRUE,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS appointments (
  id SERIAL PRIMARY KEY,
  clinic_id TEXT NOT NULL REFERENCES clinics(id) ON DELETE CASCADE,
  doctor_id INT REFERENCES doctors(id) ON DELETE SET NULL,
  patient_name TEXT NOT NULL,
  patient_phone TEXT NOT NULL,
  patient_email TEXT,
  appointment_date DATE NOT NULL,
  appointment_time TIME NOT NULL,
  duration_minutes INT DEFAULT 30,
  service TEXT,
  source TEXT DEFAULT 'whatsapp' CHECK (source IN ('whatsapp','instagram','walkin','website','phone')),
  status TEXT DEFAULT 'pending' CHECK (status IN ('pending','confirmed','cancelled','completed','no_show')),
  reminder_sent BOOLEAN DEFAULT FALSE,
  reminder_count INT DEFAULT 0,
  risk_score NUMERIC(3,2) DEFAULT 0.00,
  consent_given BOOLEAN DEFAULT FALSE,
  deposit_paid BOOLEAN DEFAULT FALSE,
  deposit_amount_sar NUMERIC(10,2) DEFAULT 0,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS otp_codes (
  id SERIAL PRIMARY KEY,
  clinic_id TEXT NOT NULL REFERENCES clinics(id) ON DELETE CASCADE,
  phone TEXT NOT NULL,
  code TEXT NOT NULL,
  purpose TEXT NOT NULL DEFAULT 'login' CHECK (purpose IN ('login','booking','verification')),
  expires_at TIMESTAMPTZ NOT NULL,
  used_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS audit_logs (
  id SERIAL PRIMARY KEY,
  clinic_id TEXT,
  actor TEXT,
  action TEXT NOT NULL,
  details JSONB DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS reminder_logs (
  id SERIAL PRIMARY KEY,
  clinic_id TEXT NOT NULL REFERENCES clinics(id) ON DELETE CASCADE,
  appointment_id INT REFERENCES appointments(id) ON DELETE SET NULL,
  channel TEXT NOT NULL CHECK (channel IN ('whatsapp','instagram','sms','email')),
  status TEXT NOT NULL DEFAULT 'queued' CHECK (status IN ('queued','sent','failed','delivered')),
  payload JSONB DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS backup_history (
  id SERIAL PRIMARY KEY,
  clinic_id TEXT REFERENCES clinics(id) ON DELETE SET NULL,
  file_name TEXT NOT NULL,
  size_bytes BIGINT DEFAULT 0,
  backup_type TEXT DEFAULT 'daily' CHECK (backup_type IN ('daily','manual','restore')),
  created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_clinic_tokens_clinic ON clinic_tokens(clinic_id);
CREATE INDEX IF NOT EXISTS idx_doctors_clinic ON doctors(clinic_id, is_active);
CREATE INDEX IF NOT EXISTS idx_appointments_clinic_date ON appointments(clinic_id, appointment_date, appointment_time);
CREATE INDEX IF NOT EXISTS idx_appointments_patient ON appointments(clinic_id, patient_phone);
CREATE INDEX IF NOT EXISTS idx_appointments_status ON appointments(clinic_id, status);
CREATE INDEX IF NOT EXISTS idx_otp_clinic_phone ON otp_codes(clinic_id, phone, expires_at);
CREATE INDEX IF NOT EXISTS idx_audit_clinic ON audit_logs(clinic_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_reminder_logs_clinic ON reminder_logs(clinic_id, created_at DESC);

INSERT INTO clinics (id, name, phone, email, city, region, status)
VALUES (
  'test_clinic',
  'عيادة النخيل',
  '+966500000000',
  'clinic@example.com',
  'الرياض',
  'الرياض',
  'online'
) ON CONFLICT (id) DO NOTHING;

INSERT INTO clinic_tokens (
  clinic_id,
  payment_token,
  whatsapp_token,
  instagram_token,
  api_token,
  otp_secret,
  otp_enabled
)
VALUES (
  'test_clinic',
  'replace-payment-token',
  'replace-whatsapp-token',
  'replace-instagram-token',
  'replace-clinic-api-token',
  'replace-otp-secret',
  TRUE
) ON CONFLICT (clinic_id) DO NOTHING;

INSERT INTO clinic_settings (
  clinic_id,
  reminder_before_minutes,
  reminder_medium,
  deposit_required,
  deposit_amount_sar,
  no_show_threshold,
  working_hours,
  backup_retention_days
)
VALUES (
  'test_clinic',
  180,
  'whatsapp',
  TRUE,
  50.00,
  0.10,
  '{"saturday_to_thursday": "09:00-17:00", "friday": "closed"}'::jsonb,
  30
) ON CONFLICT (clinic_id) DO NOTHING;

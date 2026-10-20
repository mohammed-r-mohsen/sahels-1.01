use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use argon2::{
    password_hash::{PasswordHash, SaltString},
    Argon2, PasswordHasher, PasswordVerifier,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub clinic_id: String,
    pub email: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Clone)]
pub struct AppSecurity {
    jwt_secret: String,
    encryption_key: String,
}

impl AppSecurity {
    pub fn new(jwt_secret: String, encryption_key: String) -> Self {
        Self {
            jwt_secret,
            encryption_key,
        }
    }

    pub fn jwt_secret(&self) -> &str {
        &self.jwt_secret
    }

    pub fn generate_jwt(&self, clinic_id: &str, email: &str) -> Result<String, Box<dyn std::error::Error>> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;
        let exp = now + 86400;
        let claims = TokenClaims {
            sub: email.to_string(),
            clinic_id: clinic_id.to_string(),
            email: email.to_string(),
            exp,
            iat: now,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;
        Ok(token)
    }

    pub fn verify_jwt(&self, token: &str) -> Result<TokenClaims, Box<dyn std::error::Error>> {
        let validation = Validation::new(Algorithm::HS256);
        let decoded = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &validation,
        )?;
        Ok(decoded.claims)
    }

    pub fn hash_password(&self, password: &str) -> Result<String, Box<dyn std::error::Error>> {
        let salt = SaltString::generate(&mut OsRng);
        let argon = Argon2::default();
        let hash = argon.hash_password(password.as_bytes(), &salt)?;
        Ok(hash.to_string())
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let parsed = PasswordHash::new(hash)?;
        let argon = Argon2::default();
        Ok(argon.verify_password(password.as_bytes(), &parsed).is_ok())
    }

    pub fn encrypt(&self, plain_text: &str) -> Result<String, Box<dyn std::error::Error>> {
        let key_bytes = self.decode_key()?;
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(&rand::random::<[u8; 12]>());
        let ciphertext = cipher.encrypt(nonce, plain_text.as_bytes())?;
        let mut out = Vec::with_capacity(12 + ciphertext.len());
        out.extend_from_slice(nonce.as_slice());
        out.extend_from_slice(&ciphertext);
        Ok(BASE64.encode(out))
    }

    pub fn decrypt(&self, encoded: &str) -> Result<String, Box<dyn std::error::Error>> {
        let key_bytes = self.decode_key()?;
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        let raw = BASE64.decode(encoded)?;
        let nonce = &raw[..12];
        let ciphertext = &raw[12..];
        let plaintext = cipher.decrypt(nonce.into(), ciphertext)?;
        Ok(String::from_utf8(plaintext)?)
    }

    pub fn generate_otp(&self, clinic_id: &str, phone: &str, ttl_seconds: u64) -> String {
        let current_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        let seed = format!("{clinic_id}:{phone}:{current_epoch}:{ttl_seconds}:{}", self.jwt_secret());
        let hashed = format!("{:x}", md5::compute(seed.as_bytes()));
        hashed.chars().take(6).collect::<String>()
    }

    fn decode_key(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let key_value = self.encryption_key.trim();
        let bytes = if let Some(stripped) = key_value.strip_prefix("base64:") {
            BASE64.decode(stripped)?
        } else {
            key_value.as_bytes().to_vec()
        };

        if bytes.len() != 32 {
            return Err("ENCRYPTION_KEY must resolve to 32 bytes".into());
        }

        Ok(bytes)
    }
}

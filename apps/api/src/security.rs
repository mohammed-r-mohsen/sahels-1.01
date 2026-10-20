use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use argon2::{
    password_hash::{PasswordHash, SaltString},
    Argon2, PasswordHasher, PasswordVerifier,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use rand::rngs::OsRng;
use std::error::Error;

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

    pub fn hash_password(&self, password: &str) -> Result<String, Box<dyn Error>> {
        let salt = SaltString::generate(&mut OsRng);
        let argon = Argon2::default();
        let hash = argon.hash_password(password.as_bytes(), &salt)?;
        Ok(hash.to_string())
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, Box<dyn Error>> {
        let parsed = PasswordHash::new(hash)?;
        let argon = Argon2::default();
        Ok(argon.verify_password(password.as_bytes(), &parsed).is_ok())
    }

    pub fn encrypt(&self, plain_text: &str) -> Result<String, Box<dyn Error>> {
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

    pub fn decrypt(&self, encoded: &str) -> Result<String, Box<dyn Error>> {
        let key_bytes = self.decode_key()?;
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        let raw = BASE64.decode(encoded)?;
        let nonce = &raw[..12];
        let ciphertext = &raw[12..];
        let plaintext = cipher.decrypt(nonce.into(), ciphertext)?;
        Ok(String::from_utf8(plaintext)?)
    }

    fn decode_key(&self) -> Result<Vec<u8>, Box<dyn Error>> {
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

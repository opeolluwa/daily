use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};

use crate::error::KernelError;

const KEY_LENGTH: usize = 32;
const NONCE_LENGTH: usize = 12;
const TAG_LENGTH: usize = 16;

fn parse_key(encryption_key_hex: &str) -> Result<aes_gcm::aead::Key<Aes256Gcm>, KernelError> {
    let key_bytes = hex::decode(encryption_key_hex)
        .map_err(|err| KernelError::CipherKeyDecodeError(err.to_string()))?;

    if key_bytes.len() != KEY_LENGTH {
        return Err(KernelError::CipherKeyDecodeError(format!(
            "ENCRYPTION_KEY must be exactly {KEY_LENGTH} bytes (64 hex characters), got {}",
            key_bytes.len()
        )));
    }

    let key: [u8; KEY_LENGTH] = key_bytes.try_into().map_err(|_| {
        KernelError::CipherKeyDecodeError("ENCRYPTION_KEY must be exactly 32 bytes".into())
    })?;

    Ok(key.into())
}

/// Encrypts plaintext using AES-256-GCM.
///
/// The encryption key must be a 32-byte hexadecimal-encoded key.
/// The returned ciphertext is Base64-encoded and contains the nonce
/// followed by the authenticated ciphertext.
///
/// A new random nonce is generated for each encryption.
///
/// # Errors
///
/// Returns [`KernelError::CipherKeyDecodeError`] if the encryption key
/// is invalid, or [`KernelError::CipherEncryptError`] if encryption fails.
pub fn encrypt(plaintext: &str, encryption_key: &str) -> Result<String, KernelError> {
    encrypt_with_key(encryption_key, plaintext)
}

/// Decrypts a Base64-encoded AES-256-GCM ciphertext.
///
/// The encryption key must be a 32-byte hexadecimal-encoded key.
/// The encoded data must contain a 12-byte nonce followed by the
/// authenticated ciphertext.
///
/// # Errors
///
/// Returns [`KernelError::CipherKeyDecodeError`] if the encryption key
/// is invalid, or [`KernelError::CipherDecryptError`] if the ciphertext
/// is invalid, authentication fails, or the plaintext is not valid UTF-8.
pub fn decrypt(encoded: &str, encryption_key: &str) -> Result<String, KernelError> {
    decrypt_with_key(encryption_key, encoded)
}

fn encrypt_with_key(key_hex: &str, plaintext: &str) -> Result<String, KernelError> {
    let key = parse_key(key_hex)?;
    let cipher = Aes256Gcm::new(&key);

    let nonce_bytes: [u8; NONCE_LENGTH] = rand::random();
    let nonce = Nonce::from(nonce_bytes);

    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .map_err(|_| KernelError::CipherEncryptError("encryption failed".into()))?;

    let mut combined = Vec::with_capacity(NONCE_LENGTH + ciphertext.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    Ok(BASE64.encode(combined))
}

fn decrypt_with_key(key_hex: &str, encoded: &str) -> Result<String, KernelError> {
    let data = BASE64
        .decode(encoded)
        .map_err(|_| KernelError::CipherDecryptError("invalid Base64 ciphertext".into()))?;

    if data.len() < NONCE_LENGTH + TAG_LENGTH {
        return Err(KernelError::CipherDecryptError(
            "ciphertext is too short".into(),
        ));
    }

    let (nonce_bytes, ciphertext) = data.split_at(NONCE_LENGTH);

    let nonce_array: [u8; NONCE_LENGTH] = nonce_bytes
        .try_into()
        .map_err(|_| KernelError::CipherDecryptError("invalid nonce length".into()))?;

    let nonce = Nonce::from(nonce_array);

    let key = parse_key(key_hex)?;
    let cipher = Aes256Gcm::new(&key);

    let plaintext = cipher
        .decrypt(&nonce, ciphertext)
        .map_err(|_| KernelError::CipherDecryptError("decryption failed".into()))?;

    String::from_utf8(plaintext)
        .map_err(|_| KernelError::CipherDecryptError("decrypted data is not valid UTF-8".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_KEY: &str =
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    #[test]
    fn roundtrip() {
        let plaintext = "hello, world!";
        let encrypted = encrypt(plaintext, TEST_KEY).unwrap();
        let decrypted = decrypt(&encrypted, TEST_KEY).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn different_ciphertext_each_time() {
        let first = encrypt("same", TEST_KEY).unwrap();
        let second = encrypt("same", TEST_KEY).unwrap();

        assert_ne!(first, second);
    }

    #[test]
    fn wrong_key_fails() {
        let other_key =
            "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789";

        let encrypted = encrypt("secret", TEST_KEY).unwrap();

        assert!(decrypt(&encrypted, other_key).is_err());
    }

    #[test]
    fn invalid_key_length() {
        assert!(parse_key("0123456789abcdef").is_err());
    }

    #[test]
    fn invalid_hex() {
        let invalid_key =
            "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz";

        assert!(parse_key(invalid_key).is_err());
    }

    #[test]
    fn truncated_ciphertext_fails() {
        assert!(decrypt("dG9vc2hvcnQ=", TEST_KEY).is_err());
    }
}
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use rand::RngCore;
use std::io;

const NONCE_SIZE: usize = 12;

/// Derives a 256-bit key from a password using a simple hash
/// In production, use a proper KDF like Argon2 or PBKDF2
fn derive_key(password: &str) -> [u8; 32] {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    password.hash(&mut hasher);
    let hash = hasher.finish();

    // This is NOT cryptographically secure - use proper KDF in production
    let mut key = [0u8; 32];
    for i in 0..4 {
        let bytes = ((hash >> (i * 16)) as u64).to_le_bytes();
        key[i * 8..(i + 1) * 8].copy_from_slice(&bytes);
    }
    key
}

pub fn encrypt(data: &str, password: &str) -> Result<Vec<u8>, String> {
    let key = derive_key(password);
    let cipher = Aes256Gcm::new(&key.into());

    // Generate a random nonce
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt the data
    let ciphertext = cipher
        .encrypt(nonce, data.as_bytes())
        .map_err(|e| format!("Encryption failed: {}", e))?;

    // Prepend nonce to ciphertext
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

pub fn decrypt(encrypted_data: &[u8], password: &str) -> Result<String, String> {
    if encrypted_data.len() < NONCE_SIZE {
        return Err("Invalid encrypted data".to_string());
    }

    let key = derive_key(password);
    let cipher = Aes256Gcm::new(&key.into());

    // Extract nonce and ciphertext
    let (nonce_bytes, ciphertext) = encrypted_data.split_at(NONCE_SIZE);
    let nonce = Nonce::from_slice(nonce_bytes);

    // Decrypt the data
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))?;

    String::from_utf8(plaintext).map_err(|e| format!("Invalid UTF-8: {}", e))
}

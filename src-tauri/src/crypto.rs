use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use anyhow::{Context, Result};
use argon2::{
    password_hash::{rand_core::RngCore, SaltString},
    Argon2, PasswordHasher,
};
use std::fs;
use std::path::Path;

const NONCE_SIZE: usize = 12;

/// パスワードから暗号化キーを導出
pub fn derive_key(password: &str) -> Result<Vec<u8>> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .context("Failed to hash password")?;

    // ハッシュから32バイトのキーを取得（AES-256用）
    let hash_bytes = password_hash.hash.context("No hash generated")?.as_bytes();
    Ok(hash_bytes[..32].to_vec())
}

/// データを暗号化
pub fn encrypt_data(data: &[u8], password: &str) -> Result<Vec<u8>> {
    // 固定ソルトを使用（実際にはソルトも保存すべき）
    let salt = b"knowledge_vault_";
    let mut key = [0u8; 32];

    argon2::Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .context("Failed to derive encryption key")?;

    let cipher = Aes256Gcm::new_from_slice(&key)
        .context("Failed to create cipher")?;

    // ランダムなノンスを生成
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // データを暗号化
    let ciphertext = cipher
        .encrypt(nonce, data)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

    // ノンス + 暗号文を結合
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

/// データを復号化
pub fn decrypt_data(encrypted_data: &[u8], password: &str) -> Result<Vec<u8>> {
    if encrypted_data.len() < NONCE_SIZE {
        anyhow::bail!("Encrypted data too short");
    }

    // 固定ソルトを使用
    let salt = b"knowledge_vault_";
    let mut key = [0u8; 32];

    argon2::Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .context("Failed to derive decryption key")?;

    let cipher = Aes256Gcm::new_from_slice(&key)
        .context("Failed to create cipher")?;

    // ノンスと暗号文を分離
    let nonce = Nonce::from_slice(&encrypted_data[..NONCE_SIZE]);
    let ciphertext = &encrypted_data[NONCE_SIZE..];

    // 復号化
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

    Ok(plaintext)
}

/// ファイルを暗号化
pub fn encrypt_file(input_path: &Path, output_path: &Path, password: &str) -> Result<()> {
    let data = fs::read(input_path)
        .context("Failed to read input file")?;

    let encrypted = encrypt_data(&data, password)?;

    fs::write(output_path, encrypted)
        .context("Failed to write encrypted file")?;

    Ok(())
}

/// ファイルを復号化
pub fn decrypt_file(input_path: &Path, output_path: &Path, password: &str) -> Result<()> {
    let encrypted_data = fs::read(input_path)
        .context("Failed to read encrypted file")?;

    let decrypted = decrypt_data(&encrypted_data, password)?;

    fs::write(output_path, decrypted)
        .context("Failed to write decrypted file")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let data = b"Hello, World!";
        let password = "test_password";

        let encrypted = encrypt_data(data, password).unwrap();
        let decrypted = decrypt_data(&encrypted, password).unwrap();

        assert_eq!(data.to_vec(), decrypted);
    }
}

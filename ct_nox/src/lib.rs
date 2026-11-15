pub mod ct_nox;
pub mod encrypt;
pub mod decrypt;
pub mod image_strip;

use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

pub const KDF_ITERATIONS: u32 = 600_000;
pub const SALT_LEN: usize = 16;
pub const NONCE_LEN: usize = 12;
pub const KEY_LEN: usize = 16;

pub fn derive_key(password: &str, salt: &[u8], iterations: u32) -> [u8; KEY_LEN] {
    let mut key = [0u8; KEY_LEN];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, iterations, &mut key);
    key
}

pub fn random_bytes(size: usize) -> Vec<u8> {
    (0..size).map(|_| rand::random()).collect()
}

pub fn get_valid_key(key: &str) -> Vec<u8> {
    let mut bytes = key.as_bytes().to_vec();
    if bytes.len() < 16 {
        for _j in 0..(16 - bytes.len()) {
            bytes.push(0x00);
        }
    } else if bytes.len() > 16 {
        bytes = bytes[0..16].to_vec();
    }
    bytes
}

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes128Gcm, Nonce};
use crate::{derive_key, random_bytes, KDF_ITERATIONS, NONCE_LEN, SALT_LEN};

pub fn encrypt(_text: &str, password: &str) -> String {
    let data = _text.as_bytes();
    let salt = random_bytes(SALT_LEN);
    let nonce_bytes = random_bytes(NONCE_LEN);
    let key = derive_key(password, &salt, KDF_ITERATIONS);

    let cipher = Aes128Gcm::new_from_slice(&key).unwrap();
    let nonce = Nonce::from_slice(&nonce_bytes);
    let encrypted = cipher.encrypt(nonce, data).unwrap();

    let (cipher_text, mac) = encrypted.split_at(encrypted.len() - 16);

    format!(
        "v2/pbkdf2-sha256/{}/{}/{}/{}/{}",
        KDF_ITERATIONS,
        hex::encode(salt),
        hex::encode(nonce_bytes),
        hex::encode(cipher_text),
        hex::encode(mac)
    )
}

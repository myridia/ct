use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes128Gcm, Nonce};
use std::error::Error;
use std::io::{self, ErrorKind};
use crate::{derive_key, get_valid_key, KEY_LEN, NONCE_LEN, SALT_LEN};

const MAX_KDF_ITERATIONS: u32 = 2_000_000;

pub fn decrypt(_text: &str, _password: &str) -> String {
    match _decrypt(_text, _password) {
        Ok(b) => String::from_utf8(b).unwrap_or_else(|_| _text.to_string()),
        Err(_) => _text.to_string(),
    }
}

fn _decrypt(iv_data_mac: &str, key: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    if iv_data_mac.starts_with("v2/") {
        _decrypt_v2(iv_data_mac, key)
    } else {
        _decrypt_v1(iv_data_mac, key)
    }
}

fn _decrypt_v2(orig: &str, password: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let split: Vec<&str> = orig.split('/').collect();
    if split.len() != 7 || split[0] != "v2" || split[1] != "pbkdf2-sha256" {
        return Err(Box::new(io::Error::from(ErrorKind::InvalidData)));
    }

    let iterations: u32 = split[2].parse()?;
    if iterations == 0 || iterations > MAX_KDF_ITERATIONS {
        return Err(Box::new(io::Error::from(ErrorKind::InvalidData)));
    }

    let salt = hex::decode(split[3])?;
    let nonce = hex::decode(split[4])?;
    let data = hex::decode(split[5])?;
    let mac = hex::decode(split[6])?;
    if salt.len() != SALT_LEN || nonce.len() != NONCE_LEN || mac.len() != 16 {
        return Err(Box::new(io::Error::from(ErrorKind::InvalidData)));
    }

    let key = derive_key(password, &salt, iterations);
    let cipher = Aes128Gcm::new_from_slice(&key)
        .map_err(|_| io::Error::from(ErrorKind::InvalidData))?;
    let mut cipher_text = data;
    cipher_text.extend_from_slice(&mac);
    let plain = cipher
        .decrypt(Nonce::from_slice(&nonce), cipher_text.as_slice())
        .map_err(|_| io::Error::from(ErrorKind::InvalidData))?;
    Ok(plain)
}

fn _decrypt_v1(orig: &str, password: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let split: Vec<&str> = orig.split('/').collect();
    if split.len() != 3 {
        return Err(Box::new(io::Error::from(ErrorKind::InvalidData)));
    }

    let iv = hex::decode(split[0])?;
    let data = hex::decode(split[1])?;
    let mac = hex::decode(split[2])?;
    if iv.len() != NONCE_LEN || mac.len() != 16 {
        return Err(Box::new(io::Error::from(ErrorKind::InvalidData)));
    }

    let key = get_valid_key(password);
    if key.len() != KEY_LEN {
        return Err(Box::new(io::Error::from(ErrorKind::InvalidData)));
    }

    let cipher = Aes128Gcm::new_from_slice(&key)
        .map_err(|_| io::Error::from(ErrorKind::InvalidData))?;
    let mut cipher_text = data;
    cipher_text.extend_from_slice(&mac);
    let plain = cipher
        .decrypt(Nonce::from_slice(&iv), cipher_text.as_slice())
        .map_err(|_| io::Error::from(ErrorKind::InvalidData))?;
    Ok(plain)
}

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes128Gcm, Nonce};
use crate::get_valid_key;

fn get_iv(size: usize) -> Vec<u8> {
    let mut iv = vec![];
    for _j in 0..size {
        let r = rand::random();
        iv.push(r);
    }

    iv
}

pub fn encrypt(_text: &str, password: &str) -> String {
    let data = _text.as_bytes();
    let valid_key = get_valid_key(password);
    let iv = get_iv(12);

    let cipher = Aes128Gcm::new_from_slice(&valid_key).unwrap();
    let nonce = Nonce::from_slice(&iv);
    let encrypted = cipher.encrypt(nonce, data).unwrap();

    let (cipher_text, mac) = encrypted.split_at(encrypted.len() - 16);

    let hex_iv = hex::encode(iv);
    let hex_cipher = hex::encode(cipher_text);
    let hex_mac = hex::encode(mac);

    format!("{}/{}/{}", hex_iv, hex_cipher, hex_mac)
}

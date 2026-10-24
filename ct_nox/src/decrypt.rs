use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes128Gcm, Nonce};
use std::error::Error;
use std::io::ErrorKind;
use std::str::from_utf8;
use std::{io, str};
use crate::get_valid_key;

pub fn decrypt(_text: &str, _password: &str) -> String {
    let db = _decrypt(_text, _password);
    if db.is_err() {
        return _text.to_string();
    } else {
        let b = db.unwrap();
        let s = from_utf8(&b).unwrap();
        return s.to_string();
    }
}

fn _decrypt(iv_data_mac: &str, key: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let (iv, data, mac) = split_iv_data_mac(iv_data_mac)?;
    if iv.len() != 12 {
        return Err(Box::new(io::Error::from(ErrorKind::InvalidData)));
    }
    let key = get_valid_key(key);
    let cipher = Aes128Gcm::new_from_slice(&key)
        .map_err(|_| io::Error::from(ErrorKind::InvalidData))?;
    let nonce = Nonce::from_slice(&iv);
    let mut cipher_text = data.clone();
    cipher_text.extend_from_slice(&mac);
    let plain = cipher
        .decrypt(nonce, cipher_text.as_slice())
        .map_err(|_| io::Error::from(ErrorKind::InvalidData))?;
    Ok(plain)
}

fn split_iv_data_mac(orig: &str) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let split: Vec<&str> = orig.split('/').into_iter().collect();

    if split.len() != 3 {
        return Err(Box::new(io::Error::from(ErrorKind::Other)));
    }
    let iv_res = hex::decode(split[0]);
    if iv_res.is_err() {
        return Err(Box::new(io::Error::from(ErrorKind::Other)));
    }
    let iv = iv_res.unwrap();

    let data_res = hex::decode(split[1]);
    if data_res.is_err() {
        return Err(Box::new(io::Error::from(ErrorKind::Other)));
    }
    let data = data_res.unwrap();

    let mac_res = hex::decode(split[2]);
    if mac_res.is_err() {
        return Err(Box::new(io::Error::from(ErrorKind::Other)));
    }
    let mac = mac_res.unwrap();

    Ok((iv, data, mac))
}

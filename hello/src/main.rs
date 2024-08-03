use std::error::Error;
use std::{io, str};
use std::io::ErrorKind;
use std::iter::repeat;
use std::str::from_utf8;
use crypto::aead::{AeadDecryptor, AeadEncryptor};
use crypto::aes_gcm::AesGcm;

fn main() {
    let data = "hello world";
    let password = "12345";

    println!("Data to encrypt: \"{}\" and password: \"{}\"", &data, &password);

    println!("Encrypting now");
    let res = encrypt(data.as_bytes(), password);

    
}



///encrypt "data" using "password" as the password                                                                                                                                               
/// Output is [hexNonce]/[hexCipher]/[hexMac] (nonce and iv are the same thing)                                                                                                                  
pub fn encrypt(data: &[u8], password: &str) -> String {

    let key_size = crypto::aes::KeySize::KeySize128;
        
    let valid_key = get_valid_key(password);
    let iv =vec![35, 231, 60, 170, 101, 112, 100, 43, 67, 165, 243, 7];

    println!("iv {:?}", iv);
    let mut cipher = AesGcm::new(key_size, &valid_key, &iv, &[]);

    let mut encrypted: Vec<u8> = repeat(0).take(data.len()).collect();
    println!("eeee {:?}", encrypted);
    let mut mac: Vec<u8> = repeat(0).take(16).collect();
    println!("mac {:?}", encrypted);
    cipher.encrypt(data, &mut encrypted, &mut mac[..]);
    println!("encryped: {:?}",encrypted);
    let hex_iv = hex::encode(iv);
    let hex_cipher = hex::encode(encrypted);
    let hex_mac = hex::encode(mac);
    let output = format!("{}/{}/{}", hex_iv, hex_cipher, hex_mac);
    output

//    return "xxxxx".to_string();	
}


/// gets a valid key. This must be exactly 16 bytes. if less than 16 bytes, it will be padded with 0.                                                                                            
/// If more than 16 bytes, it will be truncated                                                                                                                                                  
fn get_valid_key(key: &str) -> Vec<u8> {
    let mut bytes = key.as_bytes().to_vec();
    if bytes.len() < 16 {
        for j in 0..(16 - bytes.len()) {
            bytes.push(0x00);
        }
    } else if bytes.len() > 16 {
        bytes = bytes[0..16].to_vec();
    }
    println!("key: {:?}",bytes);
    bytes
}

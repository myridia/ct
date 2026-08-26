use std::iter::repeat;
use crypto::aead::{AeadEncryptor};
use crypto::aes_gcm::AesGcm;
use crate::get_valid_key;

/// Creates an initial vector (iv). This is also called a nonce
fn get_iv(size: usize) -> Vec<u8> {
    let mut iv = vec![];
    for _j in 0..size {
        let r = rand::random();
        iv.push(r);
    }

    iv
}

///encrypt "data" using "password" as the password
/// Output is [hexNonce]/[hexCipher]/[hexMac] (nonce and iv are the same thing)
pub fn encrypt(_text: &str, password: &str) -> String {
    let data = _text.as_bytes();
    let key_size = crypto::aes::KeySize::KeySize128;
 
    //pad or truncate the key if necessary
    let valid_key = get_valid_key(password);
    let iv = get_iv(12); //initial vector (iv), also called a nonce
    //let iv =vec![35, 231, 60, 170, 101, 112, 100, 43, 67, 165, 243, 7];
    //println!("iv {:?}", iv);
    
    let mut cipher = AesGcm::new(key_size, &valid_key, &iv, &[]);
 
    //create a vec of data.len 0's. This is where the encrypted data will be saved.
    //the encryption is performed in-place, so this vector of 0's will be converted
    //to the encrypted data
    let mut encrypted: Vec<u8> = repeat(0).take(data.len()).collect();
    //println!("eeee {:?}", encrypted); 
    //create a vec of 16 0's. This is for the mac. This library calls it a "tag", but it's really
    // the mac address. This vector will be modified in place, just like the "encrypted" vector
    // above
    let mut mac: Vec<u8> = repeat(0).take(16).collect();
    //println!("mac {:?}", encrypted);


    //encrypt data, put it into "encrypted"
    cipher.encrypt(data, &mut encrypted, &mut mac[..]);

    
    //create the output string that contains the nonce, cipher text, and mac
    //println!("encryped: {:?}",encrypted);
    let hex_iv = hex::encode(iv);
    let hex_cipher = hex::encode(encrypted);
    let hex_mac = hex::encode(mac);
    let output = format!("{}/{}/{}", hex_iv, hex_cipher, hex_mac);
 
    output
}



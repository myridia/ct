use std::error::Error;
use std::{io, str};
use std::io::ErrorKind;
use std::iter::repeat;
use crypto::aead::{AeadDecryptor};
use crypto::aes_gcm::AesGcm;
use std::str::from_utf8;


pub fn decrypt(_text: &str, _password: &str) -> String {
  let db = _decrypt(_text,_password);
  if db.is_err()
  {
    return "...cannot decrypt".to_string(); 
  }
  else
  {
   let b = db.unwrap();
   let s = from_utf8(&b).unwrap();
   return s.to_string();   
  }    
}


fn _decrypt(iv_data_mac: &str, key: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let (iv, data, mac) = split_iv_data_mac(iv_data_mac)?;
    let key = get_valid_key(key);
    let key_size = crypto::aes::KeySize::KeySize128;
    let mut decipher = AesGcm::new(key_size, &key, &iv, &[]);
    let mut dst: Vec<u8> = repeat(0).take(data.len()).collect();
    let _result = decipher.decrypt(&data, &mut dst, &mac);
    //if result {println!("{:?}",result);}
    //return Ok(Vec::new())
    return Ok(dst)	    
}



//https://boringadv.com/2022/12/05/simple-encryption-in-rust/ 
/// orig must be a string of the form [hexNonce]/[hexCipherText]/[hexMac]. This
/// is the data returned from encrypt(). This function splits the data, removes
/// the hex encoding, and returns each as a list of bytes.
fn split_iv_data_mac(orig: &str) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let split: Vec<&str> = orig.split('/').into_iter().collect();

    
    if split.len() != 3 {
        return Err(Box::new(io::Error::from(ErrorKind::Other)));
    }
    let iv_res = hex::decode(split[0]);
    if iv_res.is_err() {
        return Err(Box::new(io::Error::from(ErrorKind::Other)));
    }
    //println!("iv_res: {:?}",&iv_res);                  
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
    
    //println!("mac: {:?}",mac);
    //println!("split: {:?}",split[0]);

    //println!("iv: {:?}",iv);      
    Ok((iv, data, mac))
}
 
/// gets a valid key. This must be exactly 16 bytes. if less than 16 bytes, it will be padded with 0.
/// If more than 16 bytes, it will be truncated
fn get_valid_key(key: &str) -> Vec<u8> {
    let mut bytes = key.as_bytes().to_vec();
    if bytes.len() < 16 {
        for _j in 0..(16 - bytes.len()) {
            bytes.push(0x00);
        }
    } else if bytes.len() > 16 {
        bytes = bytes[0..16].to_vec();
    }
    //println!("key: {:?}",bytes); 
    bytes
}

 




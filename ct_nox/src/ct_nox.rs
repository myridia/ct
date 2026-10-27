use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use crate::encrypt::encrypt;


pub fn get_hello()->String {
    return "hello world".to_string();
}

//https://doc.rust-lang.org/std/fs/struct.File.html
pub fn read_file(file_path: &str)-> String
{
    let file = File::open(file_path).expect("Unable to open");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    let _f = buf_reader.read_to_string(&mut contents);
    return contents;
}


pub fn write_file(file_path: &str, contents: &str)-> std::io::Result<()>
{
  let mut file = File::create(file_path).expect("Unable to write");
  file.write_all(contents.as_bytes()).expect("Unable to write data");
  Ok(())
}

pub fn encrypt_dir(dir_path: &str, password: &str) -> Vec<String> {
    let mut saved = Vec::new();
    let entries = fs::read_dir(dir_path).expect("Unable to open");
    for entry in entries {
        let entry = entry.expect("Unable to read dir entry");
        let path = entry.path();
        let path_str = path.to_string_lossy().to_string();
        if path.is_dir() {
            saved.extend(encrypt_dir(&path_str, password));
        } else {
            let txt = read_file(&path_str);
            let ct = encrypt(&txt, password);
            let out = format!("{}.ct", path_str);
            write_file(&out, &ct).expect("Unable to write");
            saved.push(out);
        }
    }
    saved
}


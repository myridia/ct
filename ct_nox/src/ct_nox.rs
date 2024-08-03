use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


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


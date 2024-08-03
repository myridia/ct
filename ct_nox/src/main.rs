use clap::{Command, Arg};
use ct_nox::ct_nox::{read_file, write_file};
use ct_nox::encrypt::{encrypt};
use ct_nox::decrypt::{decrypt};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
// https://docs.rs/clap/4.1.4/clap/struct.Command.html
// https://docs.rs/clap/latest/clap/builder/struct.Arg.html#method.help_heading


fn main() {
    // https://docs.rs/clap/latest/clap/_tutorial/index.html
    let matches = Command::new("My Program")
    .author("Me, me@mail.com")
    .version("1.0.2")
    .about("Explains in brief what the program does")
    .arg(Arg::new("mode").short('m').long("mode"))
    .arg(Arg::new("password").short('p').long("password"))
    .arg(Arg::new("text").short('t').long("text").help("Example: ./ct_nox -f  /home/foo/bar.txt -m encrypt -p 12345" ))
    .arg(Arg::new("file").short('f').long("file").help("Example: ./ct_nox -f  /home/foo/bar.txt -m encrypt -p 12345"))
    .arg(Arg::new("output").short('o').long("output").help("Example: ./ct_nox -f  /home/foo/bar.txt -m encrypt -p 12345 -o /home/foo/bar.ct"))
    .after_help("Longer explanation to appear after the options when \
                 displaying the help information from --help or -h")
    .get_matches();
    let mut _password = "";
    let mut _text = "";
    let mut _file = "";
    let mut _output = "";        
    let mut _mode = "encrypt";    
   
    if let Some(password) = matches.get_one::<String>("password") {
       _password = password;
    }

    if let Some(text) = matches.get_one::<String>("text") {
       _text = text;
    }

    if let Some(file) = matches.get_one::<String>("file") {
       _file = file;
    }    

    if let Some(mode) = matches.get_one::<String>("mode") {
       _mode = mode;
    }

    if let Some(output) = matches.get_one::<String>("output") {
       _output = output;
     }            

    if _password != "" && _text != "" && _mode == "encrypt"
    {
	let ct = encrypt(_text,_password);
        println!("{}",ct);	
    }

    if _password != "" && _text != "" && _mode == "decrypt"
    {
	let text = decrypt(_text,_password);
        println!("{}",text);		
    }

    
    if _password != "" && _file != "" && _mode == "encrypt"
    {
      println!("...load file: {}",_file);
      let txt = read_file("/home/veto2/Downloads/hello.txt");	
      let ct = encrypt(&txt ,_password);
      if _output != ""
      {
          let r = write_file(_output, &ct);
        println!("saved to: {}",_output);	  
      }
      else
      {
        println!("{}",ct);
      }	    

    }	        


}

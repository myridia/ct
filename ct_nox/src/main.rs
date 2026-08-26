use clap::{Arg, Command};
use ct_nox::ct_nox::{read_file, write_file};
use ct_nox::decrypt::decrypt;
use ct_nox::encrypt::encrypt;
use ct_nox::image_strip::{encode_to_image, encode_to_selected_image, decode_from_image};

fn ensure_extension(path: &str, ext: &str) -> String {
    match path.rfind('.') {
        Some(pos) => {
            let existing = &path[pos..];
            if existing.eq_ignore_ascii_case(ext) {
                path.to_string()
            } else {
                format!("{}{}", path, ext)
            }
        }
        None => format!("{}{}", path, ext),
    }
}

fn main() {
    let matches = Command::new("ct_nox")
        .author("veto")
        .about("Encrypt/decrypt text with AES-128-GCM")
        .subcommand(
            Command::new("encrypt")
                .about("Encrypt text")
                .arg(Arg::new("text").short('t').long("text"))
                .arg(Arg::new("password").short('p').long("password").required(true))
                .arg(Arg::new("file").short('f').long("file"))
                .arg(Arg::new("output").short('o').long("output")),
        )
        .subcommand(
            Command::new("decrypt")
                .about("Decrypt text")
                .arg(Arg::new("text").short('t').long("text"))
                .arg(Arg::new("password").short('p').long("password").required(true))
                .arg(Arg::new("file").short('f').long("file")),
        )
        .subcommand(
            Command::new("image-encode")
                .about("Encrypt text/file and encode as binary frame PNG")
                .arg(Arg::new("text").short('t').long("text"))
                .arg(Arg::new("file").short('f').long("file"))
                .arg(Arg::new("password").short('p').long("password").required(true))
                .arg(Arg::new("output").short('o').long("output").required(true)),
        )
        .subcommand(
            Command::new("image-decode")
                .about("Decode binary frame PNG and decrypt")
                .arg(Arg::new("file").short('f').long("file").required(true))
                .arg(Arg::new("password").short('p').long("password").required(true)),
        )
        .subcommand(
            Command::new("image-encode-selected")
                .about("Encrypt and overlay frame onto an existing PNG")
                .arg(Arg::new("text").short('t').long("text"))
                .arg(Arg::new("file").short('f').long("file"))
                .arg(Arg::new("background").short('b').long("background").required(true))
                .arg(Arg::new("password").short('p').long("password").required(true))
                .arg(Arg::new("output").short('o').long("output").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("encrypt", args)) => {
            let password = args.get_one::<String>("password").unwrap();
            if let Some(text) = args.get_one::<String>("text") {
                let ct = encrypt(text, password);
                if let Some(output) = args.get_one::<String>("output") {
                    let out = ensure_extension(output, ".ct");
                    write_file(&out, &ct).unwrap();
                    println!("saved to: {}", out);
                } else {
                    println!("{}", ct);
                }
            } else if let Some(file) = args.get_one::<String>("file") {
                let txt = read_file(file);
                let ct = encrypt(&txt, password);
                if let Some(output) = args.get_one::<String>("output") {
                    let out = ensure_extension(output, ".ct");
                    write_file(&out, &ct).unwrap();
                    println!("saved to: {}", out);
                } else {
                    println!("{}", ct);
                }
            }
        }
        Some(("decrypt", args)) => {
            let password = args.get_one::<String>("password").unwrap();
            if let Some(text) = args.get_one::<String>("text") {
                let pt = decrypt(text, password);
                println!("{}", pt);
            } else if let Some(file) = args.get_one::<String>("file") {
                let ct = read_file(file);
                let pt = decrypt(&ct, password);
                println!("{}", pt);
            }
        }
        Some(("image-encode", args)) => {
            let password = args.get_one::<String>("password").unwrap();
            let output = args.get_one::<String>("output").unwrap();
            let text = if let Some(t) = args.get_one::<String>("text") {
                t.clone()
            } else if let Some(f) = args.get_one::<String>("file") {
                read_file(f)
            } else {
                eprintln!("Error: provide --text or --file");
                return;
            };
            let ct = encrypt(&text, password);
            let out = ensure_extension(output, ".png");
            if let Err(e) = encode_to_image(&ct, &out) {
                eprintln!("Error encoding image: {}", e);
            } else {
                println!("Image saved to: {}", out);
            }
        }
        Some(("image-decode", args)) => {
            let password = args.get_one::<String>("password").unwrap();
            let file = args.get_one::<String>("file").unwrap();
            match decode_from_image(file) {
                Ok(ct) => {
                    let pt = decrypt(&ct, password);
                    println!("{}", pt);
                }
                Err(e) => eprintln!("Error decoding image: {}", e),
            }
        }
        Some(("image-encode-selected", args)) => {
            let password = args.get_one::<String>("password").unwrap();
            let output = args.get_one::<String>("output").unwrap();
            let background = args.get_one::<String>("background").unwrap();
            let text = if let Some(t) = args.get_one::<String>("text") {
                t.clone()
            } else if let Some(f) = args.get_one::<String>("file") {
                read_file(f)
            } else {
                eprintln!("Error: provide --text or --file");
                return;
            };
            let ct = encrypt(&text, password);
            let out = ensure_extension(output, ".png");
            if let Err(e) = encode_to_selected_image(&ct, background, &out) {
                eprintln!("Error encoding image: {}", e);
            } else {
                println!("Image saved to: {}", out);
            }
        }
        _ => {
            eprintln!("Use --help for usage information");
        }
    }
}

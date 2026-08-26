use homedir::my_home;
use i18n_embed::DesktopLanguageRequester;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io;

use std::path::Path;

pub fn save_config(language: &str) -> bool {
    let mut config = get_config();
    config.language = language.to_string();
    write_config(&config);
    true
}

pub fn save_last_dir(dir: &str) {
    let mut config = get_config();
    config.last_dir = dir.to_string();
    write_config(&config);
}

fn write_config(config: &AppConfig) {
    let toml = toml::to_string(config).unwrap();
    let home = my_home().unwrap().unwrap();
    let config_dir = &format!("{0}/.ct/", home.display());
    let config_path = &format!("{0}/.ct/config.toml", home.display());
    let _ = fs::create_dir_all(config_dir);
    fs::write(config_path, toml).unwrap();
}

pub fn get_config() -> AppConfig {
    let config = match load_or_initialize() {
        Ok(v) => v,
        Err(err) => {
            match err {
                ConfigError::IoError(err) => {
                    eprintln!("An error occurred while loading the config: {err}");
                }
                ConfigError::InvalidConfig(err) => {
                    eprintln!("An error occurred while parsing the config:");
                    eprintln!("{err}");
                }
            }

            AppConfig {
                language: "en-US".to_string(),
                last_dir: String::new(),
            }
        }
    };
    //println!("{:?}", config);
    return config;
    //    return "xxxx".to_string();
}

enum ConfigError {
    IoError(io::Error),
    InvalidConfig(toml::de::Error),
}

impl From<io::Error> for ConfigError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(value: toml::de::Error) -> Self {
        Self::InvalidConfig(value)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub language: String,
    #[serde(default)]
    pub last_dir: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut lang = "en-US".to_string();
        let _lang = get_lang();
        if _lang.is_ok() {
            lang = _lang.unwrap();
        }
        Self {
            language: lang,
            last_dir: String::new(),
        }
    }
}

fn load_or_initialize() -> Result<AppConfig, ConfigError> {
    //  https://dev.to/zofia/why-do-we-need-configuration-creating-and-handling-configuration-files-in-rust-4a46?ysclid=momg1uxytu755103190
    let home = my_home().unwrap().unwrap();
    let _config_dir = &format!("{0}/.ct/", home.display());
    let _config_path = &format!("{0}/.ct/config.toml", home.display());
    let config_path = Path::new(_config_path);
    let config_dir = Path::new(_config_dir);
    let config = AppConfig::default();

    if config_path.exists() {
        //println!("...path exists:{}", _config_path);
        let content = fs::read_to_string(config_path)?;
        //println!(":{:?}", content);
        let config: AppConfig = toml::from_str(&content).expect("failed");
        return Ok(config);
    } else {
        let toml = toml::to_string(&config).unwrap();
        //println!(":{:?}", toml);
        let _x = fs::create_dir_all(config_dir);
        fs::write(config_path, toml)?;
    }

    //    println!(":{:?}", config.host);
    Ok(config)
}

fn get_lang() -> Result<String, Box<dyn Error>> {
    let mut r = "en-US".to_string();

    let requested_languages = DesktopLanguageRequester::requested_languages();
    let ftl: Vec<String> = env!("ftl").split(',').map(|s| s.to_string()).collect();
    let lang = requested_languages[0].language.to_string();
    let reg = requested_languages[0].region.unwrap().to_string();
    let f = format!("{}-{}", lang, reg);
    if ftl.contains(&f) {
        r = f;
    }

    return Ok(r);
}

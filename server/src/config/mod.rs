mod config;

use serde_yaml;
use std::fs::File;
use std::io::Read;
use lazy_static::lazy_static;

lazy_static! {
    static ref CONFIG: Config = load();
}

pub fn load() -> Config {
    // Check if the CONFIG_FILE_PATH environment variable is set
    let path = std::env::var("CONFIG_FILE_PATH").unwrap_or("config.yaml".into());
    println!("path = {}", path);

    // Open the file specified by the path
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("Unable to open file {}: {}", path, err),
    };

    // Read the contents of the file into a string
    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        panic!("Unable to read file {}: {}", path, err);
    }
    // Deserialize the YAML string into a Config struct
    match serde_yaml::from_str(&contents) {
        Ok(config) => config,
        Err(err) => panic!("Unable to deserialize config file {}: {}", path, err),
    }
}

pub fn get() -> &'static Config {
    &CONFIG
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub application: Application,
    pub database: DataBase,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    pub name: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataBase {
    pub url: String,
    pub username: String,
    pub password: String,
}

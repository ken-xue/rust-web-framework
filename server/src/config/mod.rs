use tokio::fs::File;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;

pub mod config;

pub async fn load_config() -> Config {
    // Check if the CONFIG_FILE_PATH environment variable is set
    let path = std::env::var("CONFIG_FILE_PATH").unwrap_or("config.yaml".into());
    println!("path = {}", path);

    // Open the file specified by the path
    let mut file = match File::open(&path).await {
        Ok(file) => file,
        Err(err) => panic!("Unable to open file {}: {}", path, err),
    };

    // Read the contents of the file into a string
    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents).await {
        panic!("Unable to read file {}: {}", path, err);
    }

    // Deserialize the YAML string into a Config struct
    match serde_yaml::from_str(&contents) {
        Ok(config) => config,
        Err(err) => panic!("Unable to deserialize config file {}: {}", path, err),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
}
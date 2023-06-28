use std::fs;
use std::path::Path;
use std::io::{self};
use reqwest;

pub const TEMPLATE_DIR: &str = "/tmp/rwf/template";
pub const DEFAULT_BASE_REMOTE_URL: &str = "https://raw.githubusercontent.com/ken-xue/rust-web-framework/main/server/crates/code/template/";

pub fn fetch_template(template_name: &str, refresh: bool) -> Result<(), Box<dyn std::error::Error>> {
    // Check if the template directory exists
    if !Path::new(TEMPLATE_DIR).exists() {
        fs::create_dir_all(TEMPLATE_DIR)?;
    }

    let template_path = format!("{}/{}", TEMPLATE_DIR, template_name);

    // Check if the template file already exists
    if Path::new(&template_path).exists() && !refresh {
        println!("Template '{}' already exists.", template_name);
        return Ok(());
    }

    let remote_url = format!("{}{}", DEFAULT_BASE_REMOTE_URL, template_name);

    // Fetch the template file from the remote URL
    let response = reqwest::blocking::get(&remote_url)?;

    if response.status().is_success() {
        let mut file = fs::File::create(&template_path)?;
        io::copy(&mut response.bytes().unwrap().as_ref(), &mut file)?;
        println!("Template '{}' fetched and saved.", template_name);
    } else {
        println!("Failed to fetch the template '{}'.", template_name);
    }

    Ok(())
}

extern crate bcrypt;
extern crate rand;

use bcrypt::{hash, verify, DEFAULT_COST};
use rand::Rng;
use std::collections::HashMap;

struct User {
    username: String,
    password: String,
}

struct Database {
    users: HashMap<String, String>,
}

impl Database {
    fn new() -> Self {
        Database {
            users: HashMap::new(),
        }
    }

    fn register_user(&mut self, username: &str, password: &str) -> Result<(), String> {
        if self.users.contains_key(username) {
            return Err("Username already exists.".to_string());
        }

        let hashed_password = match hash(password, DEFAULT_COST) {
            Ok(hashed) => hashed,
            Err(_) => return Err("Failed to hash password.".to_string()),
        };
        println!("{:?}",username);
        println!("{:?}",hashed_password);
        self.users.insert(username.to_string(), hashed_password);
        Ok(())
    }

    fn login_user(&self, username: &str, password: &str) -> Result<(), String> {
        match self.users.get(username) {
            Some(hashed_password) => {
                if verify(password, hashed_password).is_ok() {
                    Ok(())
                } else {
                    Err("Incorrect password.".to_string())
                }
            }
            None => Err("Username not found.".to_string()),
        }
    }
}

fn main() {
    let mut db = Database::new();

    // 注册用户
    let username = "alice";
    let password = "password123";
    match db.register_user(username, password) {
        Ok(()) => println!("User registered successfully."),
        Err(err) => println!("Error: {}", err),
    }

    // 尝试登录
    let login_username = "alice";
    let login_password = "password123";
    match db.login_user(login_username, login_password) {
        Ok(()) => println!("User logged in successfully."),
        Err(err) => println!("Error: {}", err),
    }
}

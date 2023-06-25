use serde::{Deserialize};
use validator::{Validate};

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateUser {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub account: String,
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateUser {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub account: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub password: String,
}
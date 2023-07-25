use crate::system::auth;

pub mod request;
pub mod response;
pub mod validator;
pub mod error;

pub fn current_username() -> String {
    let username = auth::CURRENT_USER.with(|cell| cell.borrow().clone()).map(|x| x);
    return username.unwrap_or("Null".to_string())
}
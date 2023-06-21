use std::error::Error;
use serde::Serialize;
use uuid::Uuid;
mod util;

pub fn uuid() -> String {
    let uuid = Uuid::new_v4();
    let uuid_string = uuid.to_string().replace("-", "");
    // println!("{}", uuid_string);
    return uuid_string;
}

#[derive(Serialize)]
pub struct Response<T> {
    pub code: i16,
    pub message: String,
    pub data: Option<T>,
    // pub error: String,
}

pub fn response<T: 'static + Serialize>(data: Result<T, Box<dyn Error>>) -> Response<T> {
    let result = match data {
        Ok(d) => Response {
            code : 200,
            message: "success".parse().unwrap(),
            data : Some(d),
            // error: "".to_string(),
        },
        Err(e) => Response {
            code: 500,
            message: e.to_string(),
            // error:
            data: None,
        },
    };
    return result
}

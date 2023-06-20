use std::error::Error;
use axum::response::IntoResponse;
use serde::Serialize;
use uuid::Uuid;
use serde_json::json;
mod util;

pub fn uuid() -> String {
    let uuid = Uuid::new_v4();
    let uuid_string = uuid.to_string().replace("-", "");
    // println!("{}", uuid_string);
    return uuid_string;
}

pub fn response<T: 'static + Serialize>(data: Result<T, Box<dyn Error>>) -> impl IntoResponse {
    let result = match data {
        Ok(d) => Response {
            code : 200,
            message: "Success".parse().unwrap(),
            data : Some(data.unwrap()),
            error: "".to_string(),
        },
        Err(e) => Response {
            code: 500,
            message: "Internal Server Error".parse().unwrap(),
            error: e.to_string(),
            data: None,
        },
    };
    warp::reply::json(&result)
}

#[derive(Serialize)]
pub struct Response<T> {
    pub code: i16,
    pub data: Option<T>,
    pub error: String,
    pub message: String,
}
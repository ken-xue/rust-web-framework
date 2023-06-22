use std::error::Error;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response<T> {
    pub code: i16,
    pub message: String,
    pub data: Option<T>,
}

pub fn response<T: 'static + Serialize>(data: Result<T, Box<dyn Error>>) -> Response<T> {
    let result = match data {
        Ok(d) => Response {
            code : 200,
            message: "success".parse().unwrap(),
            data : Some(d),
        },
        Err(e) => Response {
            code: 500,
            message: e.to_string(),
            data: None,
        },
    };
    return result
}

#[derive(Debug,Serialize)]
pub struct PageResponse<T> {
    pub list: Vec<T>,
    pub page: i64,
    pub size: i64,
    pub total: i64,
}

impl<T> PageResponse<T> {
    pub fn new(list: Vec<T>, page: i64, size: i64, total: i64) -> Self {
        Self {
            list,
            page,
            size,
            total,
        }
    }
}
use std::error::Error;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response<T> {
    pub code: i16,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
}

pub fn response<T: 'static + Serialize>(data: Result<T, Box<dyn Error>>) -> impl IntoResponse {
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
    (StatusCode::CREATED, Json(result))
}

pub fn error(e: Box<dyn Error>) -> impl IntoResponse {
    let response = Response {
        code: 500,
        message: e.to_string(),
        data: Some(""),
    };
    (StatusCode::OK, Json(response))
}

pub fn err(code :i16,message : String) -> impl IntoResponse {
    let response = Response {
        code,
        message,
        data: Some(""),
    };
    (StatusCode::OK, Json(response))
}

pub fn success<T: Serialize>(data: T) -> impl IntoResponse {
    (StatusCode::OK, Json(Response {
        code : 200,
        message: "success".parse().unwrap(),
        data : Some(data),
    }))
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
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Delete {
    pub ids: Vec<u64>,
}

#[derive(Deserialize)]
pub struct Page {
    pub page: i64,
    pub size: i64,
}
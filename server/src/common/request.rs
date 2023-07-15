use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Delete {
    pub ids: Vec<u64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub page: i64,
    pub page_size: i64,
}
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationRequest {
    pub page: i64,
    pub page_size: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginatedResult<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64
}
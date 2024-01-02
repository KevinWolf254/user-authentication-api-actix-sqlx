use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PaginationRequest {
    pub page: i64,
    pub page_size: i64,
}

#[derive(Serialize)]
pub struct PaginatedResult<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64
}
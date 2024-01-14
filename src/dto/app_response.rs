use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AppResponse<'a> {
    pub message: &'a str,
}

impl AppResponse<'static> {
    pub fn new(message: &'static str) -> Self {
        AppResponse {
            message
        }
    }
}
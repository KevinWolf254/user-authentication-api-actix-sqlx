use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Organisation {
    pub organisation_id: u32,
    pub name: String,
    pub email_address: String,
    pub address: String,
    pub code: String,
}
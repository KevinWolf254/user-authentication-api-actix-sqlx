use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Unit {
    pub unit_id: u32,
    pub amount: u32,
    pub organisation_id: u32,
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub user_id: u32,
    pub first_name: String,
    pub middle_name: String,
    pub surname: String,
    pub email_address: String,
    pub mobile_number: String,
    pub enabled: bool,
    pub email_confirmed: bool,
    pub organisation_id: u32,
}
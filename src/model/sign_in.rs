use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignIn {
    pub email_address: String,
    pub password: String,
}
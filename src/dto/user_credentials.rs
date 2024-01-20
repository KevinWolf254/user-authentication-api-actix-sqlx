use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserCredential {
    #[validate(length(min = 3, message = "Username is required!"))]
    pub username: String,
    #[validate(length(min = 3, message = "Password is required!"))]
    pub password: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateUserCredential {
    #[validate(length(min = 3, message = "Password is required!"))]
    pub password: String,
}
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    #[validate(length(min = 3, message = "First name is required!"))]
    pub first_name: String,
    pub middle_name: Option<String>,
    #[validate(length(min = 3, message = "Surname is required!"))]
    pub surname: String,
    #[validate(email(message = "Email address is not valid!"))]
    pub email_address: String,
    pub mobile_number: Option<String>,
}

#[derive(Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUser {
    #[validate(length(min = 3, message = "First name is required!"))]
    pub first_name: String,
    pub middle_name: Option<String>,
    #[validate(length(min = 3, message = "Surname is required!"))]
    pub surname: String,
    pub mobile_number: Option<String>,
}
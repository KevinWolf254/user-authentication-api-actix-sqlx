use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignUp {
    #[validate(length(min = 3, message = "First name is required!"))]
    pub first_name: String,
    #[validate(length(min = 3, message = "Surname is required!"))]
    pub surname: String,
    #[validate(email(message = "Email address is not valid!"))]
    pub email_address: String,
    #[validate(length(min = 3, message = "Password is required!"))]
    pub password: String,
}
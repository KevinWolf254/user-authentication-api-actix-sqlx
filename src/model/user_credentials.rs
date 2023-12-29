use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use regex::Regex;
use validator::Validate;

static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9A-Za-z_]+$").unwrap());

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserCredentials {
    pub user_credential_id: u32,
    
    #[validate(length(min = 3, max = 16), regex = "USERNAME_REGEX")]
    pub username: String,
    
    pub password: String,
    
    pub user_id: u32,
}
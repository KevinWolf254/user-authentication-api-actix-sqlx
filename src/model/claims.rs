use serde::{Deserialize, Serialize};

use crate::entity::{permission::Permission, role::Role, user::User};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub user: User,
    pub role: Role,
    pub permissions: Vec<Permission>,
    pub iat: usize,
    pub exp: usize,
}
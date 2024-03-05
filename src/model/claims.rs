use serde::{Deserialize, Serialize};

use crate::entity::role::Role;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub roles: Vec<Role>,
    pub iat: usize,
    pub exp: usize,
}
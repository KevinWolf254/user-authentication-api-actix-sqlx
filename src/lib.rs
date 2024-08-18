use std::sync::Arc;

use argon2::Config;
use dao::Database;

pub mod handler;
pub mod entity;
pub mod dao;
pub mod error;
pub mod model;
pub mod util;
pub mod jwt;
pub mod auth;
pub mod email;

pub struct AppState<'a> {
    pub context: Arc<Database<'a>>,
    pub argon_config: Arc<Config<'a>>,
    pub jwt_config: Arc<JwtConfig>,
}

pub struct JwtConfig {
    pub secret: String,
    pub expires_in: i64,
}
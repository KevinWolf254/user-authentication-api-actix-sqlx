use std::sync::{Mutex, Arc};

use argon2::Config;
use dao::Database;
use model::jwt_config::JwtConfig;

pub mod handler;
pub mod entity;
pub mod dao;
pub mod error;
pub mod model;
pub mod util;
pub mod jwt;

pub struct AppState<'a> {
    pub connections: Mutex<u32>,
    pub context: Arc<Database<'a>>,
    pub argon_config: Arc<Config<'a>>,
    pub jwt_config: Arc<JwtConfig>,
}
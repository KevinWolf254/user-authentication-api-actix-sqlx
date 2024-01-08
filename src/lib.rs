use std::sync::{Mutex, Arc};

use dao::Database;
use slog::Logger;

pub mod handler;
pub mod entity;
pub mod dao;
pub mod error;
pub mod dto;

pub struct AppState<'a> {
    pub connections: Mutex<u32>,
    pub context: Arc<Database<'a>>,
    pub log: Logger
}

use std::{sync::{Mutex, Arc}, fs::OpenOptions};

use dao::Database;
use slog::{Logger, Drain, o};

pub mod handler;
pub mod entity;
pub mod dao;
pub mod error;
pub mod dto;

pub const DEFAULT_LOG_PATH: &str = "log/sms_gateway.log";

pub struct AppState<'a> {
    pub connections: Mutex<u32>,
    pub context: Arc<Database<'a>>,
    pub log: Arc<Logger>
}

pub fn configure_log(log_path: String) -> Logger {
    let file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(log_path)
      .unwrap();
    let decorator = slog_term::PlainDecorator::new(file);
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!())
}
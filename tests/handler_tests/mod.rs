use std::{sync::{Mutex, Arc}, env};

use actix_web::web::{self, Data};
use bulk_sms_api::{dao::Database, AppState, configure_log, DEFAULT_LOG_PATH};
use dotenvy::dotenv;
use sqlx::Pool;

#[cfg(test)]
mod permission_handler_test;

pub async fn init_app_state(pool: Pool<sqlx::Postgres>) -> Data<AppState<'static>> {
    dotenv().ok();

    let log_path = env::var("LOG_PATH").unwrap_or_else(|_| DEFAULT_LOG_PATH.to_string());

    let log = configure_log(log_path);

    let db_context = Database::test(pool).await;

    web::Data::new(AppState {
        connections: Mutex::new(0),
        context: Arc::new(db_context),
        log: Arc::new(log.clone())
    })
}
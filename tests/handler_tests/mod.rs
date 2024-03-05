use std::{sync::{Mutex, Arc}, env};
use argon2::Config;

use actix_web::web::{self, Data};
use bulk_sms_api::{configure_log, dao::Database, model::jwt_config::JwtConfig, AppState, DEFAULT_LOG_PATH};
use dotenvy::dotenv;
use sqlx::Pool;

#[cfg(test)]
mod permission_handler_test;
#[cfg(test)]
mod role_handler_test;
#[cfg(test)]
mod user_handler_test;
#[cfg(test)]
mod auth_handler_test;

pub async fn init_app_state(pool: Pool<sqlx::Postgres>) -> Data<AppState<'static>> {
    dotenv().ok();

    let log_path = env::var("LOG_PATH").unwrap_or_else(|_| DEFAULT_LOG_PATH.to_string());

    let log = configure_log(log_path);

    let db_context = Database::test(pool).await;

    let config = Config::default();

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET was not provided.");
    let expires_in = env::var("JWT_EXPIRES_IN").expect("JWT_EXPIRES_IN was not provided.").parse::<i64>().expect("JWT_EXPIRES_IN should be an i32.");

    let jwt_config = JwtConfig {secret, expires_in};
    
    web::Data::new(AppState {
        connections: Mutex::new(0),
        context: Arc::new(db_context),
        log: Arc::new(log.clone()),
        argon_config: Arc::new(config),
        jwt_config: Arc::new(jwt_config)
    })
}
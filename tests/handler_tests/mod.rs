use std::{sync::Arc, env};
use argon2::Config;

use actix_web::web::{self, Data};
use bulk_sms_api::{dao::Database, entity::{permission::Permission, role::Role, user::User}, error::AppError, jwt, AppState, JwtConfig};
use chrono::Utc;
//configure_log,
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

    let db_context = Database::test(pool).await;

    let config = Config::default();

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET was not provided.");
    let expires_in = env::var("JWT_EXPIRES_IN").expect("JWT_EXPIRES_IN was not provided.").parse::<i64>().expect("JWT_EXPIRES_IN should be an i32.");

    let jwt_config = JwtConfig {secret, expires_in};
    
    web::Data::new(AppState {
        context: Arc::new(db_context),
        argon_config: Arc::new(config),
        jwt_config: Arc::new(jwt_config),
    })
}

pub async fn generate_token(config: &JwtConfig) -> Result<String , AppError> {
    let user = User {
        user_id: 1,
        first_name: "John".into(),
        middle_name: Some("Smith".into()),
        surname: "Smith".into(),
        email_address: "jsmith@test.com".into(),
        mobile_number: None,
        enabled: true,
        email_confirmed: true,
        role_id: 1,
        created_at: Utc::now(),

    };

    let role = Role {
        role_id: 1,
        name: "SUPER_ADMIN".into(),
        created_at: Utc::now(),
    };

    let read = Permission {
        permission_id: 1,
        name: "PERMISSION_READ".into(),
        created_at: Utc::now(),
    };

    let write = Permission {
        permission_id: 1,
        name: "PERMISSION_WRITE".into(),
        created_at: Utc::now(),
    };

    let update = Permission {
        permission_id: 1,
        name: "PERMISSION_UPDATE".into(),
        created_at: Utc::now(),
    };

    let delete = Permission {
        permission_id: 1,
        name: "PERMISSION_DELETE".into(),
        created_at: Utc::now(),
    };
    
    jwt::generate_token(user, role, vec![read, write, update, delete], config).await
}
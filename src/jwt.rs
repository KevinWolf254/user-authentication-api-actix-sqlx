use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};

use crate::{entity::{permission::Permission, role::Role, user::User}, error::{AppError, AppErrorType}, model::{claims::Claims, jwt_config::JwtConfig}};

pub async fn generate_token(user: User, role: Role, permissions: Vec<Permission>, config: &JwtConfig) -> Result<String , AppError> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(config.expires_in)).timestamp() as usize;

    let claims: Claims = Claims {
        sub: user.email_address.to_string(),
        user,
        role,
        permissions,
        exp,
        iat,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_ref()),
    )
    .map_err(|e| {
        AppError::new(None, Some(e.to_string()), AppErrorType::UnAuthorisedError)
    })
}

pub fn validate_token(token:&str, config: &JwtConfig) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_ref()),
        &Validation::default(),
    )
    .map(|r| r.claims)
    .map_err(|e| {
        AppError::new(None, Some(e.to_string()), AppErrorType::UnAuthorisedError)
    })
}


#[cfg(test)]
mod jwt_tests {
    // use super::*;

}
use actix_web::{post, web::{Data, Json, ServiceConfig}, HttpResponse};
use log::error;

use crate::{error::{AppError, AppErrorType}, jwt, model::{sign_in::SignIn, token_response::TokenResponse}, util, AppState};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(sign_in);
}

#[post("sign-in")]
pub async fn sign_in(state: Data<AppState<'_>>, body: Json<SignIn>) -> Result<HttpResponse, AppError> {
    let SignIn { email_address, password }= body.into_inner();

    let user = state.context.users.find_by_email_address(&email_address).await
    .map_err(|error| {
        error!("Error occured: {:?}", error); 
        match error {
            sqlx::Error::RowNotFound => AppError::new(Some("Invalid email address/password!".to_string()), None, AppErrorType::UnAuthorisedError),
            _  => AppError::new(None, Some("Service unavailable try again later!".to_string()), AppErrorType::InternalServerError)
        }
    })?;

    let user_credentials = state.context.user_credentials.find_by_user_id(&user.user_id).await
    .map_err(|error| {
        error!("Error occured: {:?}", error); 
        match &error {
            sqlx::Error::Database(d) if d.code().map_or(false, |code| code.eq("23503")) => {
                AppError::new(Some("Invalid email address/password!".to_string()), None, AppErrorType::UnAuthorisedError)
            },
            sqlx::Error::RowNotFound => AppError::new(Some("Invalid email address/password!".to_string()), None, AppErrorType::UnAuthorisedError),
            _ => AppError::new(None, Some("Service unavailable try again later!".to_string()), AppErrorType::InternalServerError),
        }
    })?;

    let user_role = state.context.roles.find_by_id(&user.role_id).await
    .map_err(|error| {
        error!("Error occured: {:?}", error); 
        AppError::new(None, Some("Service unavailable try again later!".to_string()), AppErrorType::InternalServerError)
    })?;

    let permissions = state.context.role_permissions.find_role_permissions(&user.role_id).await
    .map_err(|error| {
        error!("Error occured: {:?}", error); 
        AppError::new(None, Some("Service unavailable try again later!".to_string()), AppErrorType::InternalServerError)
    })?;

    let is_pass_correct = util::verify_password(&user_credentials.password, &password).await?;

    if !is_pass_correct {
        Err(AppError::new(None, Some("Invalid email address/password!".to_string()), AppErrorType::UnAuthorisedError))
    } else {
        jwt::generate_token(user, user_role, permissions, &state.jwt_config).await
        .map(|token| HttpResponse::Ok().json(TokenResponse { token }))
    }

}
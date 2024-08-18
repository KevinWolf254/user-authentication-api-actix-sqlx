use actix_web::{post, web::{Path, Data, Json, ServiceConfig}, HttpResponse};
use log::error;

use crate::{error::{AppError, AppErrorType}, jwt, model::{sign_in::SignIn, sign_up::SignUp, token_response::TokenResponse, user::CreateUser, user_credentials::CreateUserCredential}, util, AppState};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(sign_in);
    cfg.service(sign_up);
}

#[post("sign-in")]
pub async fn sign_in(state: Data<AppState<'_>>, body: Json<SignIn>) -> Result<HttpResponse, AppError> {
    let SignIn { email_address, password } = body.into_inner();

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

#[post("sign-up")]
pub async fn sign_up(state: Data<AppState<'_>>, body: Json<SignUp>) -> Result<HttpResponse, AppError> {
    let SignUp { first_name, surname, email_address, password} = body.into_inner();

    let create = CreateUser {
        first_name,
        middle_name: None,
        surname,
        email_address: email_address.clone(),
        mobile_number: None,
        role_id: 1 // TODO - should be for basic user role
    };

    let user = state.context.users.create(&create).await
    .map(|user| user)
    .map_err(|error| {
        error!("Error occured: {:?}", error); 
        match &error {
            sqlx::Error::Database(d) if d.code().map_or(false, |code| code.eq("23505")) => {
                AppError::new(Some("Email address already exists!".to_string()), None, AppErrorType::BadRequestError)
            }
            _ => AppError::new(None, Some(error.to_string()), AppErrorType::InternalServerError),
        }
    })?;
    let splits: Vec<&str> = user.email_address.split('@').collect();
    let username = splits.get(0).ok_or_else(|| AppError::new(None, Some("Email address is invalid!".to_string()), AppErrorType::InternalServerError))?;

    let hashed_password = util::hash_password(&password, &state.argon_config).await?;
    
    state.context.user_credentials.create(&user.user_id, &CreateUserCredential{ username: username.to_string(), password: hashed_password }).await
    .map_err(|error| {
        error!("Error occured: {:?}", error); 
        match &error {
            sqlx::Error::Database(d) if d.code().map_or(false, |code| code.eq("23503")) => {
                AppError::new(Some(format!("User with id {} could not be found!", user.user_id)), None, AppErrorType::NotFoundError)
            },
            sqlx::Error::Database(d) if d.code().map_or(false, |code| code.eq("23505")) => {
                AppError::new(Some("Credential/username already exists!".to_string()), None, AppErrorType::BadRequestError)
            }
            _ => AppError::new(None, Some(error.to_string()), AppErrorType::InternalServerError),
        }
    })?;

    let code = util::generate_confirmation_code().await;

    state.context.user_code.create(&user.user_id, &code).await
    .map_err(|error| {
        error!("Error occured: {:?}", error); 
        match &error {
            sqlx::Error::Database(d) if d.code().map_or(false, |code| code.eq("23503")) => {
                AppError::new(Some(format!("User with id {} could not be found!", user.user_id)), None, AppErrorType::NotFoundError)
            },
            sqlx::Error::Database(d) if d.code().map_or(false, |code| code.eq("23505")) => {
                AppError::new(Some("Credential/username already exists!".to_string()), None, AppErrorType::BadRequestError)
            }
            _ => AppError::new(None, Some(error.to_string()), AppErrorType::InternalServerError),
        }
    })?;

    // TODO - send email to email gateway

    Ok(HttpResponse::Created().json(user))
    
}

#[post("sign-up/{user_id}/verify/{code}")]
pub async fn confirm_email_address(state: Data<AppState<'_>>, path: Path<(i32, i32)>) -> Result<HttpResponse, AppError> {
    // find email address and respective code
    // check that the code has not expired
    // if expired generate new code and resend
    // else if code matches
    // save user to user table and credentials to credentials table
    // remove from temp_user table
    let (user_id, code) = path.into_inner();

    state.context.user_code.find_by_user_id_and_code(&user_id, &code).await
    .map_err(|error| {
        error!("Error occured: {:?}", error); 
        match &error {
            sqlx::Error::Database(d) if d.code().map_or(false, |code| code.eq("23503")) => {
                AppError::new(Some(format!("User with id {} and code {} could not be found!", &user_id, &code)), None, AppErrorType::NotFoundError)
            },
            _ => AppError::new(None, Some(error.to_string()), AppErrorType::InternalServerError),
        }
    })?;

    let mut user = state.context.users.find_by_id(&user_id).await.map_err(|error| {
        error!("Error occured: {:?}", error); 
        match &error {
            sqlx::Error::Database(d) if d.code().map_or(false, |code| code.eq("23503")) => {
                AppError::new(Some(format!("User with id {} could not be found!", &user_id)), None, AppErrorType::NotFoundError)
            },
            _ => AppError::new(None, Some(error.to_string()), AppErrorType::InternalServerError),
        }
    })?;

    if user.email_confirmed {
        return Err(AppError::new(None, Some(format!("User with id {} already confirmed email!", &user_id)), AppErrorType::BadRequestError));
    }

    user.email_confirmed = true;
    user.enabled = true;

    user = state.context.users.update_user(&user).await
    .map(|user| user)
    .map_err(|error| {
        error!("Error occured: {:?}", error); 
        match &error {
            sqlx::Error::Database(d) if d.code().map_or(false, |code| code.eq("23503")) => {
                AppError::new(Some(format!("User with id {} could not be found!", &user_id)), None, AppErrorType::NotFoundError)
            },
            _ => AppError::new(None, Some(error.to_string()), AppErrorType::InternalServerError),
        }
    })?;

    state.context.user_code.delete(&user_id, &code).await
    .map_err(|error| {
        error!("Error occured: {:?}", error); 
        AppError::new(None, Some(error.to_string()), AppErrorType::InternalServerError)
    })?;

    // generate json web token

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

    jwt::generate_token(user, user_role, permissions, &state.jwt_config).await
    .map(|token| HttpResponse::Ok().json(TokenResponse { token }))
}
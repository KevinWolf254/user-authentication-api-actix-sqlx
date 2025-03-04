use actix_web::{ delete, get, post, put, web::{ Data, Path, ServiceConfig, Query }, HttpResponse };
use log::error;
use sqlx::Error::RowNotFound;
use crate::{ auth::JwtAuthenticationGuard, error::{AppError, AppErrorType, AppResponseError}, model::{app_response::AppResponse, pagination::PaginationRequest, user::{CreateUser, UpdateUser}, user_credentials::{CreateUserCredential, UpdateUserCredential}}, util, AppState };
use actix_web_validator::Json;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(get_users);
    cfg.service(get_user_by_id);
    cfg.service(get_users_paginated);
    cfg.service(create_user);
    cfg.service(update_user);
    cfg.service(delete_user_with_id);
    cfg.service(create_user_credential);
    cfg.service(update_user_credential);
}

#[get("users/{user_id}")]
pub async fn get_user_by_id(state: Data<AppState<'_>>, path: Path<i32>, _: JwtAuthenticationGuard) -> Result<HttpResponse , AppError> {
    let user_id = path.into_inner();
    state.context.users.find_by_id(&user_id).await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|error| {
            error!("Error occured: {:?}", error); 
            match error {
                RowNotFound => AppError::new(Some(format!("User with id {} could not be found!", user_id)), None, AppErrorType::NotFoundError),
                _  => AppError::new(None, Some(error.to_string()), AppErrorType::InternalServerError)
            }
        })
}

#[get("users")]
pub async fn get_users(state: Data<AppState<'_>>, _: JwtAuthenticationGuard) -> Result<HttpResponse , AppError> {
    state.context.users.find_all().await
        .map(|users| HttpResponse::Ok().json(users))
        .map_err(|error| {
                    error!("Error occured: {:?}", error); 
                    AppError::new(None, Some(error.to_string()), AppErrorType::InternalServerError)
        })
}

#[get("users-paginated")]
pub async fn get_users_paginated(state: Data<AppState<'_>>, pagination: Query<PaginationRequest>, _: JwtAuthenticationGuard) -> Result<HttpResponse , AppError> {
    state.context.users.find_paginated(pagination.page, pagination.page_size).await
        .map(|results| HttpResponse::Ok().json(results))
        .map_err(|e| {
            error!("Error occured: {:?}", e); 
            AppError::new(None, Some(e.to_string()), AppErrorType::InternalServerError)
        })
}

#[post("users")]
pub async fn create_user(state: Data<AppState<'_>>, body: Json<CreateUser>, _: JwtAuthenticationGuard) -> Result<HttpResponse , AppError>  {
    state.context.users.create(&body.into_inner()).await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|error| {
            error!("Error occured: {:?}", error); 
            match &error {
                sqlx::Error::Database(d) if d.code().map_or(false, |code| code.eq("23505")) => {
                    AppError::new(Some("User already exists!".to_string()), None, AppErrorType::BadRequestError)
                }
                _ => AppError::new(None, Some(error.to_string()), AppErrorType::InternalServerError),
            }
        })
}

#[post("users/{user_id}/credentials")]
pub async fn create_user_credential(state: Data<AppState<'_>>, path: Path<i32>, body: Json<CreateUserCredential>, _: JwtAuthenticationGuard) -> Result<HttpResponse , AppError>  {
    let user_id = path.into_inner();
    let CreateUserCredential { username, password }= body.into_inner();

    let hashed_password = util::hash_password(&password, &state.argon_config).await?;

    state.context.user_credentials.create(&user_id, &CreateUserCredential{ username, password: hashed_password }).await
        .map(|_| HttpResponse::Created().json(AppResponse { message: "Successfully created!" }))
        .map_err(|error| {
            error!("Error occured: {:?}", error); 
            match &error {
                sqlx::Error::Database(d) if d.code().map_or(false, |code| code.eq("23503")) => {
                    AppError::new(Some(format!("User with id {} could not be found!", user_id)), None, AppErrorType::NotFoundError)
                },
                sqlx::Error::Database(d) if d.code().map_or(false, |code| code.eq("23505")) => {
                    AppError::new(Some("Credential/username already exists!".to_string()), None, AppErrorType::BadRequestError)
                }
                _ => AppError::new(None, Some(error.to_string()), AppErrorType::InternalServerError),
            }
        })
}

#[put("users/{user_id}/credentials/{user_credential_id}")]
pub async fn update_user_credential(state: Data<AppState<'_>>, path: Path<(i32, i32)>, body: Json<UpdateUserCredential>, _: JwtAuthenticationGuard) -> Result<HttpResponse , AppError>  {
    let (user_id, user_credential_id) = path.into_inner();
    let UpdateUserCredential { previous_password, password } = body.into_inner();

    let user_credential = state.context.user_credentials.find_by_user_id(&user_id).await
        .map_err(|error| {
            error!("Error occured: {:?}", error); 
            match &error {
                sqlx::Error::Database(d) if d.code().map_or(false, |code| code.eq("23503")) => {
                    AppError::new(Some("Credential does not exist!".to_string()), None, AppErrorType::BadRequestError)
                },
                sqlx::Error::RowNotFound => AppError::new(Some("Credential does not exist!".to_string()), None, AppErrorType::NotFoundError),
                _ => AppError::new(None, Some(error.to_string()), AppErrorType::InternalServerError),
            }
        })?;
        
    let is_correct = util::verify_password(&user_credential.password, &previous_password).await?;

    if !is_correct {
        return Err(AppError::new(Some("Credential do match!".to_string()), None, AppErrorType::BadRequestError))
    }

    state.context.user_credentials.update(&user_id, &user_credential_id, &UpdateUserCredential{previous_password, password}).await
        .map(|_| HttpResponse::Ok().json(AppResponse { message: "Successfully updated!" }))
        .map_err(|error| {
            error!("Error occured: {:?}", error); 
            match &error {
                sqlx::Error::Database(d) if d.code().map_or(false, |code| code.eq("23503")) => {
                    AppError::new(Some("Credential does not exist!".to_string()), None, AppErrorType::BadRequestError)
                },
                sqlx::Error::RowNotFound => AppError::new(Some("Credential does not exist!".to_string()), None, AppErrorType::NotFoundError),
                _ => AppError::new(None, Some(error.to_string()), AppErrorType::InternalServerError),
            }
        })
}

#[put("users/{user_id}")]
pub async fn update_user(state: Data<AppState<'_>>, path: Path<i32>, body: Json<UpdateUser>, _: JwtAuthenticationGuard) -> Result<HttpResponse , AppError>  {
    let user_id = path.into_inner();
    state.context.users.update(&user_id, &body.into_inner()).await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|error| {
            error!("Error occured: {:?}", error); 
            AppError::new(Some(format!("User with id {} could not be found!", user_id)), None, AppErrorType::NotFoundError)
        })
}

#[delete("users/{user_id}")]
pub async fn delete_user_with_id(state: Data<AppState<'_>>, path: Path<i32>, _: JwtAuthenticationGuard) -> Result<HttpResponse , AppError> {
    let user_id = path.into_inner();
    
    state.context.users.delete(&user_id).await
        .map(|result| {
            if result.rows_affected() == 0 {
                HttpResponse::NotFound().json(AppResponseError::new(format!("User with id {} could not be found!", user_id)))
            } else {
                HttpResponse::Ok().json(AppResponse::new("User deleted successfully."))
            }
        })
        .map_err(|error| {
            error!("Error occured: {:?}", error); 
            AppError::new(None, Some(error.to_string()), AppErrorType::InternalServerError)
        })
}
use actix_web::{ delete, get, post, put, web::{ Data, Path, ServiceConfig, Query }, HttpResponse };
use slog::error;
use sqlx::Error::RowNotFound;
use crate::{ AppState, error::{AppError, AppErrorType, AppResponseError}, dto::{pagination::PaginationRequest, app_response::AppResponse, user::{CreateUser, UpdateUser}, user_credentials::{CreateUserCredential, UpdateUserCredential}} };
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
pub async fn get_user_by_id(state: Data<AppState<'_>>, path: Path<i32>) -> Result<HttpResponse , AppError> {
    let user_id = path.into_inner();
    state.context.users.find_by_id(&user_id).await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|error| {
            error!(state.log, "Error occured: {:?}", error); 
            match error {
                RowNotFound => AppError::new(Some(format!("User with id {} could not be found!", user_id)), None, AppErrorType::NotFoundError),
                _  => AppError::new(None, Some(error.to_string()), AppErrorType::DBError)
            }
        })
}

#[get("users")]
pub async fn get_users(state: Data<AppState<'_>>) -> Result<HttpResponse , AppError> {
    state.context.users.find_all().await
        .map(|users| HttpResponse::Ok().json(users))
        .map_err(|error| {
                    error!(state.log, "Error occured: {:?}", error); 
                    AppError::new(None, Some(error.to_string()), AppErrorType::DBError)
        })
}

#[get("users-paginated")]
pub async fn get_users_paginated(state: Data<AppState<'_>>, pagination: Query<PaginationRequest>) -> Result<HttpResponse , AppError> {
    state.context.users.find_paginated(pagination.page, pagination.page_size).await
        .map(|results| HttpResponse::Ok().json(results))
        .map_err(|e| {
            error!(state.log, "Error occured: {:?}", e); 
            AppError::new(None, Some(e.to_string()), AppErrorType::DBError)
        })
}

#[post("users")]
pub async fn create_user(state: Data<AppState<'_>>, body: Json<CreateUser>) -> Result<HttpResponse , AppError>  {
    state.context.users.create(&body.into_inner()).await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|error| {
            error!(state.log, "Error occured: {:?}", error); 
            match &error {
                sqlx::Error::Database(d) if d.code().map_or(false, |code| code.eq("23505")) => {
                    AppError::new(Some("User already exists!".to_string()), None, AppErrorType::BadRequestError)
                }
                _ => AppError::new(None, Some(error.to_string()), AppErrorType::DBError),
            }
        })
}

#[post("users/{user_id}/credentials")]
pub async fn create_user_credential(state: Data<AppState<'_>>, path: Path<i32>, body: Json<CreateUserCredential>) -> Result<HttpResponse , AppError>  {
    let user_id = path.into_inner();
    state.context.user_credentials.create(&user_id, &body.into_inner()).await
        .map(|_| HttpResponse::Created().json(AppResponse { message: "Successfully created!" }))
        .map_err(|error| {
            error!(state.log, "Error occured: {:?}", error); 
            match &error {
                sqlx::Error::Database(d) if d.code().map_or(false, |code| code.eq("23503")) => {
                    AppError::new(Some(format!("User with id {} could not be found!", user_id)), None, AppErrorType::NotFoundError)
                },
                sqlx::Error::Database(d) if d.code().map_or(false, |code| code.eq("23505")) => {
                    AppError::new(Some("Credential/username already exists!".to_string()), None, AppErrorType::BadRequestError)
                }
                _ => AppError::new(None, Some(error.to_string()), AppErrorType::DBError),
            }
        })
}

#[put("users/{user_id}/credentials/{user_credential_id}")]
pub async fn update_user_credential(state: Data<AppState<'_>>, path: Path<(i32, i32)>, body: Json<UpdateUserCredential>) -> Result<HttpResponse , AppError>  {
    let (user_id, user_credential_id) = path.into_inner();
    state.context.user_credentials.update(&user_id, &user_credential_id, &body.into_inner()).await
        .map(|_| HttpResponse::Ok().json(AppResponse { message: "Successfully updated!" }))
        .map_err(|error| {
            error!(state.log, "Error occured: {:?}", error); 
            match &error {
                sqlx::Error::Database(d) if d.code().map_or(false, |code| code.eq("23503")) => {
                    AppError::new(Some("Credential does not exist!".to_string()), None, AppErrorType::BadRequestError)
                },
                sqlx::Error::RowNotFound => AppError::new(Some("Credential does not exist!".to_string()), None, AppErrorType::NotFoundError),
                _ => AppError::new(None, Some(error.to_string()), AppErrorType::DBError),
            }
        })
}

#[put("users/{user_id}")]
pub async fn update_user(state: Data<AppState<'_>>, path: Path<i32>, body: Json<UpdateUser>) -> Result<HttpResponse , AppError>  {
    let user_id = path.into_inner();
    state.context.users.update(&user_id, &body.into_inner()).await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|error| {
            error!(state.log, "Error occured: {:?}", error); 
            AppError::new(Some(format!("User with id {} could not be found!", user_id)), None, AppErrorType::NotFoundError)
        })
}

#[delete("users/{user_id}")]
pub async fn delete_user_with_id(state: Data<AppState<'_>>, path: Path<i32>) -> Result<HttpResponse , AppError> {
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
            error!(state.log, "Error occured: {:?}", error); 
            AppError::new(None, Some(error.to_string()), AppErrorType::DBError)
        })
}
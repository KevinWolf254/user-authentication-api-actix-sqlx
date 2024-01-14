use actix_web::{ delete, get, post, web::{ Data, Path, ServiceConfig, Query, Json }, HttpResponse };
use slog::error;

use crate::{AppState, error::{AppError, AppErrorType, AppResponseError}, dto::{pagination::PaginationRequest, app_response::AppResponse}, entity::role::CreateRole};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(get_roles);
    cfg.service(get_role_by_id);
    cfg.service(get_roles_paginated);
    cfg.service(create_role);
    cfg.service(delete_role_with_id);
    cfg.service(get_role_permissions);
}

#[get("roles")]
pub async fn get_roles(state: Data<AppState<'_>>) -> Result<HttpResponse, AppError> {
    match state.context.roles.find_all().await {
        Ok(roles) => Ok(HttpResponse::Ok().json(roles)),
        Err(error) => {
            error!(state.log, "Error occured: {:?}", error); 
            Err(AppError::new(None, Some(error.to_string()), AppErrorType::DBError))             
        },
    }
}

#[get("roles-paginated")]
pub async fn get_roles_paginated(state: Data<AppState<'_>>, pagination: Query<PaginationRequest>) -> Result<HttpResponse , AppError> {
    state.context.roles.find_paginated(pagination.page, pagination.page_size).await
        .map(|results| HttpResponse::Ok().json(results))
        .map_err(|e| {
            error!(state.log, "Error occured: {:?}", e); 
            AppError::new(None, Some(e.to_string()), AppErrorType::DBError)
        })
}

#[get("roles/{role_id}")]
pub async fn get_role_by_id(state: Data<AppState<'_>>, path: Path<i16>) -> Result<HttpResponse , AppError> {
    let role_id = path.into_inner();
    state.context.roles.find_by_id(&role_id).await
        .map(|role| HttpResponse::Ok().json(role))
        .map_err(|error| {
            error!(state.log, "Error occured: {:?}", error); 
            match error {
                sqlx::Error::RowNotFound => AppError::new(Some(format!("Role with id {} could not be found!", role_id)), None, AppErrorType::NotFoundError),
                _  => AppError::new(None, Some(error.to_string()), AppErrorType::DBError)
            }
        })
}

#[post("roles")]
pub async fn create_role(state: Data<AppState<'_>>, body: Json<CreateRole>) -> Result<HttpResponse , AppError>  {
    state.context.roles.create(&body.into_inner()).await
        .map(|role| HttpResponse::Created().json(role))
        .map_err(|error| {
            error!(state.log, "Error occured: {:?}", error); 
            match &error {
                sqlx::Error::Database(d) if d.code().map_or(false, |code| code.eq("23505")) => {
                    AppError::new(Some("Role already exists!".to_string()), None, AppErrorType::BadRequestError)
                }
                _ => AppError::new(None, Some(error.to_string()), AppErrorType::DBError),
            }
        })
}

#[delete("roles/{role_id}")]
pub async fn delete_role_with_id(state: Data<AppState<'_>>, path: Path<i16>) -> Result<HttpResponse , AppError> {
    let role_id = path.into_inner();
    
    state.context.roles.delete(&role_id).await
        .map(|result| {
            if result.rows_affected() == 0 {
                HttpResponse::NotFound().json(AppResponseError::new(format!("Role with id {} could not be found!", role_id)))
            } else {
                HttpResponse::Ok().json(AppResponse::new("Role deleted successfully."))
            }
        })
        .map_err(|error| {
            error!(state.log, "Error occured: {:?}", error); 
            AppError::new(None, Some(error.to_string()), AppErrorType::DBError)
        })
}

#[get("roles/{role_id}/permissions")]
pub async fn get_role_permissions(state: Data<AppState<'_>>, path: Path<i16>) -> Result<HttpResponse , AppError> {
    let role_id = path.into_inner();
    state.context.role_permissions.find_role_permissions(&role_id).await
        .map(|roles| HttpResponse::Ok().json(roles))
        .map_err(|error| {
            error!(state.log, "Error occured: {:?}", error); 
            match error {
                sqlx::Error::RowNotFound => AppError::new(Some(format!("Role with id {} could not be found!", role_id)), None, AppErrorType::NotFoundError),
                _  => AppError::new(None, Some(error.to_string()), AppErrorType::DBError)
            }
        })
}
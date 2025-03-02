use actix_web::{ delete, get, post, web::{ Data, Path, ServiceConfig, Query }, HttpResponse };
use log::error;
use sqlx::Error::RowNotFound;
use crate::{ auth::JwtAuthenticationGuard, entity::permission::CreatePermission, error::{AppError, AppErrorType, AppResponseError}, model::{app_response::AppResponse, pagination::PaginationRequest}, AppState };
use actix_web_validator::Json;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(get_permissions);
    cfg.service(get_permission_by_id);
    cfg.service(get_permissions_paginated);
    cfg.service(create_permission);
    cfg.service(delete_permission_with_id);
}

#[get("permissions")]
pub async fn get_permissions(state: Data<AppState<'_>>, _: JwtAuthenticationGuard) -> Result<HttpResponse , AppError> {
    match state.context.permissions.find_all().await {
        Ok(permissions) => Ok(HttpResponse::Ok().json(permissions)),
        Err(error) => {
            error!("Error occured: {:?}", error); 
            Err(AppError::new(None, Some(error.to_string()), AppErrorType::InternalServerError))             
        },
    }
}

#[get("permissions-paginated")]
pub async fn get_permissions_paginated(state: Data<AppState<'_>>, pagination: Query<PaginationRequest>, _: JwtAuthenticationGuard) -> Result<HttpResponse , AppError> {
    state.context.permissions.find_paginated(pagination.page, pagination.page_size).await
        .map(|results| HttpResponse::Ok().json(results))
        .map_err(|e| {
            error!("Error occured: {:?}", e); 
            AppError::new(None, Some(e.to_string()), AppErrorType::InternalServerError)
        })
}

#[get("permissions/{permission_id}")]
pub async fn get_permission_by_id(state: Data<AppState<'_>>, path: Path<i16>, _: JwtAuthenticationGuard) -> Result<HttpResponse , AppError> {
    let permission_id = path.into_inner();
    state.context.permissions.find_by_id(&permission_id).await
        .map(|permission| HttpResponse::Ok().json(permission))
        .map_err(|error| {
            error!("Error occured: {:?}", error); 
            match error {
                RowNotFound => AppError::new(Some(format!("Permission with id {} could not be found!", permission_id)), None, AppErrorType::NotFoundError),
                _  => AppError::new(None, Some(error.to_string()), AppErrorType::InternalServerError)
            }
        })
}

#[post("permissions")]
pub async fn create_permission(state: Data<AppState<'_>>, body: Json<CreatePermission>, _: JwtAuthenticationGuard) -> Result<HttpResponse , AppError>  {
    state.context.permissions.create(&body.into_inner()).await
        .map(|permission| HttpResponse::Created().json(permission))
        .map_err(|error| {
            error!("Error occured: {:?}", error); 
            match &error {
                sqlx::Error::Database(d) if d.code().map_or(false, |code| code.eq("23505")) => {
                    AppError::new(Some("Permission already exists!".to_string()), None, AppErrorType::BadRequestError)
                }
                _ => AppError::new(None, Some(error.to_string()), AppErrorType::InternalServerError),
            }
        })
}

#[delete("permissions/{permission_id}")]
pub async fn delete_permission_with_id(state: Data<AppState<'_>>, path: Path<i16>, _: JwtAuthenticationGuard) -> Result<HttpResponse , AppError> {
    let permission_id = path.into_inner();
    
    state.context.permissions.delete(&permission_id).await
        .map(|result| {
            if result.rows_affected() == 0 {
                HttpResponse::NotFound().json(AppResponseError::new(format!("Permission with id {} could not be found!", permission_id)))
            } else {
                HttpResponse::Ok().json(AppResponse::new("Permission deleted successfully."))
            }
        })
        .map_err(|error| {
            error!("Error occured: {:?}", error); 
            AppError::new(None, Some(error.to_string()), AppErrorType::InternalServerError)
        })
}
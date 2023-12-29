// use super::AppState;
use actix_web::{
    delete,
    get,
    post,
    web::{ Data, Path, ServiceConfig },
    HttpResponse,
    Responder,
};
use chrono::{Utc, DateTime};
use sqlx;
use crate::{ AppState, model::permission::{Permission, CreatePermission} };
use actix_web_validator::Json;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(get_permissions);
    cfg.service(get_permission_by_id);
    cfg.service(create_permission);
    cfg.service(delete_permission_with_id);
}

#[get("permissions")]
pub async fn get_permissions(state: Data<AppState>) -> impl Responder {
    match
        sqlx::query_as!(Permission, r#"SELECT * FROM "SMS_GATEWAY_USER"."PERMISSION" "#).fetch_all(&state.db).await
    {
        Ok(permissons) => HttpResponse::Ok().json(permissons),
        Err(_) => {
            let body: Vec<Permission> = Vec::with_capacity(0);
            HttpResponse::Ok().json(body)
        },
    }
}

#[get("permissions/{permission_id}")]
pub async fn get_permission_by_id(state: Data<AppState>, path: Path<i16>) -> HttpResponse {
    let permission_id = path.into_inner();
    match
        sqlx::query_as!(Permission, r#"SELECT * FROM "SMS_GATEWAY_USER"."PERMISSION" WHERE permission_id = $1 "#, permission_id).fetch_one(&state.db).await
    {
        Ok(permisson) => HttpResponse::Ok().json(permisson),        
        Err(e) => { 
            println!("Error occured {:?}", e.to_string());
            HttpResponse::NotFound().json(format!("Permission with id {} could not be found!", permission_id))
        },
    }
}

#[post("permissions")]
pub async fn create_permission(state: Data<AppState>, body: Json<CreatePermission>) -> HttpResponse {
    let now: DateTime<Utc> = Utc::now();
    match
        sqlx::query_as!(Permission, r#"INSERT INTO "SMS_GATEWAY_USER"."PERMISSION" (name, created_at) VALUES ($1, $2) RETURNING permission_id, name, created_at"#,
            &body.name, now).fetch_one(&state.db).await
    {
        Ok(permisson) => HttpResponse::Ok().json(permisson),
        Err(e) => {
            println!("Error occured: {}", e.to_string());
            HttpResponse::InternalServerError().json(e.to_string())
        },
    }
}

#[delete("permissions/{permission_id}")]
pub async fn delete_permission_with_id(state: Data<AppState>, path: Path<i16>) -> HttpResponse {
    let permission_id = path.into_inner();
    match
        sqlx::query_as!(Permission, r#"DELETE FROM "SMS_GATEWAY_USER"."PERMISSION" WHERE permission_id = $1 "#, &permission_id).execute(&state.db).await
    {
        Ok(_) => HttpResponse::Ok().body("Permission deleted successfully!"),
        Err(_) => HttpResponse::NotFound().json(format!("Permission with id {} could not be found!", permission_id)),
    }
}
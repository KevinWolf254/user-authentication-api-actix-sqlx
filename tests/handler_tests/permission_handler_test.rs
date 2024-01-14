use actix_web::{test, App, http};
use bulk_sms_api::{handler, entity::permission::{CreatePermission, Permission}, error::AppResponseError, dto::app_response::AppResponse};
use sqlx::Pool;
use serde_json::json;

use crate::handler_tests::init_app_state;

#[sqlx::test]
pub async fn get_permissions_returns_ok(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_permission_handler),
    )
    .await;

    let request = test::TestRequest::get().uri("/permissions").to_request();

    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::OK);
}

#[sqlx::test]
pub async fn get_permissions_paginated_returns_ok(pool: Pool<sqlx::Postgres>) {
    // given
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_permission_handler),
    )
    .await;
    
    // when
    let request = test::TestRequest::get()
        .uri("/permissions-paginated?page=1&pageSize=5")
        .to_request();

    // then
    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::OK);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("permission")))]
pub async fn get_permission_by_id_returns_ok_when_id_exists(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_permission_handler),
    )
    .await;

    // given
    // when
    let request = test::TestRequest::get().uri("/permissions/1").to_request();

    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::OK);

    let body = test::read_body(response).await;
    let permission: Permission = serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(permission.permission_id, 1);
    assert_eq!(permission.name, "PERMISSION_READ");

}

#[sqlx::test]
pub async fn get_permission_by_id_returns_not_found_when_id_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_permission_handler),
    )
    .await;

    let request = test::TestRequest::get().uri("/permissions/201").to_request();

    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);

    let body = test::read_body(response).await;

    let error: AppResponseError = serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(error.error, "Permission with id 201 could not be found!");
}

#[sqlx::test]
pub async fn create_permission_returns_ok_when_name_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_permission_handler),
    )
    .await;

    let name = "PERMISSION_READ";
    let body = CreatePermission {
        name: name.to_string()
    };

    let payload = json!(body);

    let request = test::TestRequest::post().uri("/permissions")
    .set_json(&payload)
    .to_request();

    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::CREATED);

    let body = test::read_body(response).await;
    let permission: Permission =     serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(permission.permission_id, 1);
    assert_eq!(permission.name, name);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("permission")))]
pub async fn create_permission_returns_bad_request_when_name_exists(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_permission_handler),
    )
    .await;

    let name = "PERMISSION_READ";
    let body = CreatePermission {
        name: name.to_string()
    };

    let payload = json!(body);

    let request = test::TestRequest::post().uri("/permissions")
    .set_json(&payload)
    .to_request();

    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);

    let body = test::read_body(response).await;
    let result: AppResponseError =     serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(result.error, "Permission already exists!");
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("permission")))]
pub async fn delete_permission_with_id_returns_ok_when_id_exists(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;

    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_permission_handler),
    )
    .await;

    // given
    let permission_id = 1;

    // when
    let request = test::TestRequest::delete().uri(format!("/permissions/{}", permission_id).as_str()).to_request();

    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::OK);

    let body = test::read_body(response).await;

    let result: AppResponse = serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(result.message, "Permission deleted successfully.");
    // then
}

#[sqlx::test]
pub async fn delete_permission_with_id_returns_not_found_when_id_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;

    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_permission_handler),
    )
    .await;

    // given
    let permission_id = 1;

    // when
    let request = test::TestRequest::delete().uri(format!("/permissions/{}", permission_id).as_str()).to_request();

    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);

    let body = test::read_body(response).await;

    let result: AppResponseError = serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(result.error, "Permission with id 1 could not be found!");
    // then
}
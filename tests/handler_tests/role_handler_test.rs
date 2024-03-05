use actix_web::{test, App, http};
use bulk_sms_api::{handler, entity::{role::{Role, CreateRole}, permission::Permission}, error::AppResponseError, model::app_response::AppResponse};
use serde_json::json;
use sqlx::Pool;

use crate::handler_tests::init_app_state;

#[sqlx::test]
pub async fn get_roles_returns_ok(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_role_handler),
    )
    .await;

    let request = test::TestRequest::get().uri("/roles").to_request();

    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::OK);
}

#[sqlx::test]
pub async fn get_roles_paginated_returns_ok(pool: Pool<sqlx::Postgres>) {
    // given
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_role_handler),
    )
    .await;
    
    // when
    let request = test::TestRequest::get()
        .uri("/roles-paginated?page=1&pageSize=5")
        .to_request();

    // then
    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::OK);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("role")))]
pub async fn get_role_by_id_returns_ok_when_id_exists(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_role_handler),
    )
    .await;

    // given
    // when
    let request = test::TestRequest::get().uri("/roles/1").to_request();

    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::OK);

    let body = test::read_body(response).await;
    let role: Role = serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(role.role_id, 1);
    assert_eq!(role.name, "SUPER_ADMIN");

}

#[sqlx::test]
pub async fn get_role_by_id_returns_not_found_when_id_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_role_handler),
    )
    .await;

    let request = test::TestRequest::get().uri("/roles/201").to_request();

    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);

    let body = test::read_body(response).await;

    let error: AppResponseError = serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(error.error, "Role with id 201 could not be found!");
}

#[sqlx::test]
pub async fn create_role_returns_ok_when_name_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_role_handler),
    )
    .await;

    let name = "ROLE_READ";
    let body = CreateRole {
        name: name.to_string()
    };

    let payload = json!(body);

    let request = test::TestRequest::post().uri("/roles")
        .set_json(&payload)
        .to_request();

    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::CREATED);

    let body = test::read_body(response).await;
    let role: Role =     serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(role.role_id, 1);
    assert_eq!(role.name, name);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("role")))]
pub async fn create_role_returns_bad_request_when_name_exists(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_role_handler),
    )
    .await;

    let name = "ROLE_READ";
    let body = CreateRole {
        name: name.to_string()
    };

    let payload = json!(body);

    let request = test::TestRequest::post().uri("/roles")
    .set_json(&payload)
    .to_request();

    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);

    let body = test::read_body(response).await;
    let result: AppResponseError =     serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(result.error, "Role already exists!");
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("role")))]
pub async fn delete_role_with_id_returns_ok_when_id_exists(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;

    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_role_handler),
    )
    .await;

    // given
    let role_id = 1;

    // when
    let request = test::TestRequest::delete().uri(format!("/roles/{}", role_id).as_str()).to_request();

    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::OK);

    let body = test::read_body(response).await;

    let result: AppResponse = serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(result.message, "Role deleted successfully.");
}

#[sqlx::test]
pub async fn delete_role_with_id_returns_not_found_when_id_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;

    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_role_handler),
    )
    .await;

    // given
    let role_id = 1;

    // when
    let request = test::TestRequest::delete().uri(format!("/roles/{}", role_id).as_str()).to_request();

    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);

    let body = test::read_body(response).await;

    let result: AppResponseError = serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(result.error, "Role with id 1 could not be found!");
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("role", "permission", "role_permission")))]
pub async fn get_role_permissions_returns_ok(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_role_handler),
    )
    .await;
    // given
    // when
    let request = test::TestRequest::get().uri("/roles/1/permissions").to_request();

    let response = test::call_service(&mut app, request).await;

    // then
    assert_eq!(response.status(), http::StatusCode::OK);

    let body = test::read_body(response).await;

    let result: Vec<Permission> = serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(result.len(), 4);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("role")))]
pub async fn get_role_permissions_returns_ok_even_when_role_does_not_have_permissions(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_role_handler),
    )
    .await;
    // given
    // when
    let request = test::TestRequest::get().uri("/roles/1/permissions").to_request();

    let response = test::call_service(&mut app, request).await;

    // then
    assert_eq!(response.status(), http::StatusCode::OK);

    let body = test::read_body(response).await;

    let result: Vec<Permission> = serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(result.len(), 0);
}

#[sqlx::test]
pub async fn get_role_permissions_returns_not_found_when_role_id_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_role_handler),
    )
    .await;
    // given
    // when
    let request = test::TestRequest::get().uri("/roles/101/permissions").to_request();

    let response = test::call_service(&mut app, request).await;

    // then
    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);

    let body = test::read_body(response).await;

    let error: AppResponseError = serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(error.error, "Role with id 101 could not be found!");
}
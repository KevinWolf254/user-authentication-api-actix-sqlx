use actix_web::{test, App, http};
use bulk_sms_api::{handler, entity::user::User, error::AppResponseError, dto::{app_response::AppResponse, pagination::PaginatedResult, create_user::{CreateUser, UpdateUser}}};
use sqlx::Pool;
use serde_json::json;

use crate::handler_tests::init_app_state;

#[sqlx::test(fixtures(path = "../fixtures", scripts("user")))]
pub async fn get_user_by_id_returns_ok_when_id_exists(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_user_handler),
    )
    .await;

    // given
    // when
    let request = test::TestRequest::get().uri("/users/1").to_request();

    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::OK);

    let body = test::read_body(response).await;
    let user: User = serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(user.user_id, 1);
    assert_eq!(user.first_name, "John");
    assert_eq!(user.middle_name, None);
    assert_eq!(user.surname, "Smith");
    assert_eq!(user.email_address, "jsmith@test.com");
    assert_eq!(user.mobile_number, None);
}

#[sqlx::test]
pub async fn get_user_by_id_returns_not_found_when_id_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_user_handler),
    )
    .await;

    // given
    // when
    let request = test::TestRequest::get().uri("/users/1").to_request();

    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);

    let body = test::read_body(response).await;
    let response: AppResponseError = serde_json::from_slice(&body).expect("Failed to deserialize error");

    dbg!("{:?}", &response);
    assert_eq!(response.error, "User with id 1 could not be found!".to_string());
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("user")))]
pub async fn get_users_returns_ok(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_user_handler),
    )
    .await;

    let request = test::TestRequest::get().uri("/users").to_request();

    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::OK);

    let body = test::read_body(response).await;
    let response: Vec<User> = serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(response.len(), 2);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("user")))]
pub async fn get_users_paginated_returns_ok(pool: Pool<sqlx::Postgres>) {
    // given
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_user_handler),
    )
    .await;
    
    // when
    let request = test::TestRequest::get()
        .uri("/users-paginated?page=1&pageSize=5")
        .to_request();

    // then
    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::OK);

    let body = test::read_body(response).await;
    let response: PaginatedResult<User> = serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(response.page, 1);
    assert_eq!(response.page_size, 5);
    assert_eq!(response.total, 2);

}

#[sqlx::test]
pub async fn create_user_returns_ok_when_name_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_user_handler),
    )
    .await;

    // given
    let body = CreateUser {
        first_name: "John".to_string(),
        middle_name: None,
        surname: "Doe".to_string(),
        email_address: "jsmith@test.com".to_string(),
        mobile_number: None,
    };

    let payload = json!(body);

    // when
    let request = test::TestRequest::post().uri("/users")
        .set_json(&payload)
        .to_request();

    let response = test::call_service(&mut app, request).await;

    // then
    assert_eq!(response.status(), http::StatusCode::CREATED);

    let body = test::read_body(response).await;
    let user: User = serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(user.user_id, 1);
    assert_eq!(user.first_name, "John");
    assert_eq!(user.middle_name, None);
    assert_eq!(user.surname, "Doe");
    assert_eq!(user.email_address, "jsmith@test.com");
    assert_eq!(user.mobile_number, None);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("user")))]
pub async fn create_user_returns_bad_request_when_email_address_exists(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_user_handler),
    )
    .await;

    // given
    let body = CreateUser {
        first_name: "John".to_string(),
        middle_name: None,
        surname: "Doe".to_string(),
        email_address: "jsmith@test.com".to_string(),
        mobile_number: None,
    };

    let payload = json!(body);

    // when
    let request = test::TestRequest::post().uri("/users")
        .set_json(&payload)
        .to_request();

    let response = test::call_service(&mut app, request).await;

    // then
    assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);

    let body = test::read_body(response).await;
    let result: AppResponseError = serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(result.error, "User already exists!");
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("user")))]
pub async fn update_user_returns_ok(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_user_handler),
    )
    .await;

    // given
    let body = UpdateUser {
        first_name: "Jane".to_string(),
        middle_name: Some("Pope".to_string()),
        surname: "Dope".to_string(),
        mobile_number: Some("0700000000".to_string()),
    };

    let payload = json!(body);

    // when
    let request = test::TestRequest::put().uri("/users/1")
        .set_json(&payload)
        .to_request();

    let response = test::call_service(&mut app, request).await;

    // then
    assert_eq!(response.status(), http::StatusCode::OK);

    let body = test::read_body(response).await;
    let user: User = serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(user.user_id, 1);
    assert_eq!(user.first_name, "Jane");
    assert_eq!(user.middle_name.unwrap(), "Pope");
    assert_eq!(user.surname, "Dope");
    assert_eq!(user.mobile_number.unwrap(), "0700000000");
}

#[sqlx::test]
pub async fn update_user_returns_not_found_when_id_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_user_handler),
    )
    .await;

    // given
    let body = UpdateUser {
        first_name: "Jane".to_string(),
        middle_name: Some("Pope".to_string()),
        surname: "Dope".to_string(),
        mobile_number: Some("0700000000".to_string()),
    };

    let payload = json!(body);

    // when
    let request = test::TestRequest::put().uri("/users/1")
        .set_json(&payload)
        .to_request();

    let response = test::call_service(&mut app, request).await;

    // then
    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);

    let body = test::read_body(response).await;
    let response: AppResponseError = serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(response.error, "User with id 1 could not be found!");
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("user")))]
pub async fn delete_user_with_id_returns_ok_when_id_exists(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;

    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_user_handler),
    )
    .await;

    // when
    let request = test::TestRequest::delete().uri("/users/1").to_request();

    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::OK);

    let body = test::read_body(response).await;

    // then
    let result: AppResponse = serde_json::from_slice(&body).expect("Failed to deserialize error");

    assert_eq!(result.message, "User deleted successfully.");
}

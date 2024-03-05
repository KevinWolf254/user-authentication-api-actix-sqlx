use actix_web::{test, App, http};
use bulk_sms_api::{handler, model::{sign_in::SignIn, token_response::TokenResponse, user_credentials::CreateUserCredential}, util};
use sqlx::Pool;

use crate::handler_tests::init_app_state;

#[sqlx::test]
pub async fn sign_in_returns_unauthorised_when_email_address_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_auth_handler),
    )
    .await;

    let payload = SignIn{email_address: "jsmith@test.com".to_string(), password: "1234567".to_string()};

    let request = test::TestRequest::post().uri("/sign-in")
        .set_json(&payload)
        .to_request();

    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("role", "user")))]
pub async fn sign_in_returns_unauthorised_when_credentials_do_not_exist(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_auth_handler),
    )
    .await;

    let payload = SignIn{email_address: "jsmith@test.com".to_string(), password: "1234567".to_string()};

    let request = test::TestRequest::post().uri("/sign-in")
        .set_json(&payload)
        .to_request();

    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("role", "user")))]
pub async fn sign_in_returns_unauthorised_when_password_does_not_match(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;

    let user_id = 1;
    let password =  "1234567".to_string();

    let hashed_password = util::hash_password(&password, &app_state.argon_config).await.unwrap();

    app_state.context.user_credentials.create(&user_id, &CreateUserCredential{ username: "tester".to_string(), password: hashed_password }).await.unwrap();

    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_auth_handler),
    )
    .await;

    let payload = SignIn{email_address: "jsmith@test.com".to_string(), password: "wrong_password".to_string()};

    let request = test::TestRequest::post().uri("/sign-in")
        .set_json(&payload)
        .to_request();

    let response = test::call_service(&mut app, request).await;

    dbg!(&response);

    assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("role", "user")))]
pub async fn sign_in_returns_ok(pool: Pool<sqlx::Postgres>) {
    let app_state = init_app_state(pool).await;
    
    let user_id = 1;
    let password =  "1234567".to_string();

    let hashed_password = util::hash_password(&password, &app_state.argon_config).await.unwrap();

    app_state.context.user_credentials.create(&user_id, &CreateUserCredential{ username: "tester".into(), password: hashed_password }).await.unwrap();

    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(handler::init_auth_handler),
    )
    .await;

    let payload = SignIn{email_address: "jsmith@test.com".to_string(), password};

    let request = test::TestRequest::post().uri("/sign-in")
        .set_json(&payload)
        .to_request();

    let response = test::call_service(&mut app, request).await;

    dbg!(&response);

    assert_eq!(response.status(), http::StatusCode::OK);

    let body = test::read_body(response).await;

    let result: TokenResponse = serde_json::from_slice(&body).expect("Failed to deserialize error");

    dbg!("Token: ", &result.token);

    assert!(!result.token.is_empty());
}
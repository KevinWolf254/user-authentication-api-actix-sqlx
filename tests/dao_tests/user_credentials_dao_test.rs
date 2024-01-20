use bulk_sms_api::{dao::Database, dto::user_credentials::{CreateUserCredential, UpdateUserCredential}};
use sqlx::Pool;

#[sqlx::test(fixtures(path = "../fixtures", scripts("user")))]
pub async fn create_returns_a_user_credential(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let user_id = 1;
    let credentials = CreateUserCredential {
        username: "tester".to_string(),
        password: "1234567".to_string()
    };

    // when
    let result = db.user_credentials.create(&user_id, &credentials).await;

    // then
    assert!(result.is_ok());

    let result = result.unwrap();

    assert!(result.user_credential_id.is_positive());
    assert_eq!(result.username, "tester");
    assert_eq!(result.password, "1234567");
}

#[sqlx::test]
pub async fn create_returns_error_when_user_id_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let user_id = 1;
    let credentials = CreateUserCredential {
        username: "tester".to_string(),
        password: "1234567".to_string()
    };

    // when
    let result = db.user_credentials.create(&user_id, &credentials).await;

    dbg!("{:?}", &result);

    // then
    assert!(result.is_err());
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("user", "user_credential")))]
pub async fn create_returns_error_when_username_already_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let user_id = 2;
    let credentials = CreateUserCredential {
        username: "tester".to_string(),
        password: "1234567".to_string()
    };

    // when
    let result = db.user_credentials.create(&user_id, &credentials).await;

    dbg!("{:?}", &result);

    // then
    assert!(result.is_err());
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("user", "user_credential")))]
pub async fn create_returns_error_when_user_already_has_credential(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let user_id = 1;
    let credentials = CreateUserCredential {
        username: "tester1".to_string(),
        password: "1234567".to_string()
    };

    // when
    let result = db.user_credentials.create(&user_id, &credentials).await;

    dbg!("{:?}", &result);

    // then
    assert!(result.is_err());
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("user", "user_credential")))]
pub async fn update_returns_a_user_credential(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given 
    let user_id = 1;   
    let user_credential_id = 1;   
    let request = UpdateUserCredential {
        password: "newpassword".to_string(),
    };

    // when
    let result = db.user_credentials.update(&user_id, &user_credential_id, &request).await;

    // then
    assert!(result.is_ok());

    let update_user = result.unwrap();

    assert_eq!(update_user.user_id, user_id);
    assert_eq!(update_user.password, "newpassword");
}

#[sqlx::test]
pub async fn update_returns_a_error_when_user_credential_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given 
    let user_id = 1;   
    let user_credential_id = 1;   
    let request = UpdateUserCredential {
        password: "newpassword".to_string(),
    };

    // when
    let result = db.user_credentials.update(&user_id, &user_credential_id, &request).await;

    dbg!(":?", &result);
    // then
    assert!(result.is_err());
}
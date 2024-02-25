use bulk_sms_api::{dao::Database, dto::user::{CreateUser, UpdateUser}};
use sqlx::Pool;

#[sqlx::test(fixtures(path = "../fixtures", scripts("user")))]
pub async fn find_by_id_returns_user_when_id_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    let user_id = 1;
    // when
    let result = db.users.find_by_id(&user_id).await;
    
    // then
    assert!(result.is_ok());

    let created_user = result.unwrap();

    assert!(created_user.user_id.is_positive());
}

#[sqlx::test]
pub async fn find_by_id_returns_error_when_id_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let user_id = 2001;

    // when
    let result = db.users.find_by_id(&user_id).await;

    // then
    assert!(result.is_err());
}


#[sqlx::test(fixtures(path = "../fixtures", scripts("user")))]
pub async fn find_all_returns_users_when_users_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    // when
    let result = db.users.find_all().await;

    // then
    assert!(result.is_ok());

    let users = result.unwrap();

    assert_eq!(users.len(), 2);
}

#[sqlx::test]
pub async fn find_all_returns_empty_when_users_do_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // when
    let result = db.users.find_all().await;

    // then
    assert!(result.is_ok());

    let users = result.unwrap();

    assert_eq!(users.len(), 0);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("user")))]
pub async fn find_paginated_returns_paginated_result_when_users_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let page = 1;
    let page_size = 5;

    // when
    let result = db.users.find_paginated(page, page_size).await;

    // then
    assert!(result.is_ok());

    let result = result.unwrap();

    dbg!("{:?}", &result);

    assert_eq!(result.data.len(), 2);
    assert_eq!(result.total, 2);
    assert_eq!(result.page, page);
    assert_eq!(result.page_size, page_size);
}

#[sqlx::test]
pub async fn create_returns_a_user_when_user_name_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given    
    let user = CreateUser {
        first_name: "John".to_string(),
        middle_name: None,
        surname: "Doe".to_string(),
        email_address: "jsmith@test.com".to_string(),
        mobile_number: None,
    };

    // when
    let result = db.users.create(&user).await;

    // then
    assert!(result.is_ok());

    let created_user = result.unwrap();

    assert!(created_user.user_id.is_positive());
    assert_eq!(created_user.first_name, user.first_name);
    assert_eq!(created_user.middle_name, None);
    assert_eq!(created_user.surname, user.surname);
    assert_eq!(created_user.email_address, user.email_address);
    assert_eq!(created_user.mobile_number, None);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("user")))]
pub async fn create_returns_an_error_when_user_email_address_already_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given   
    let user = CreateUser {
        first_name: "John".to_string(),
        middle_name: None,
        surname: "Doe".to_string(),
        email_address: "jsmith@test.com".to_string(),
        mobile_number: None,
    };

    // when
    let result = db.users.create(&user).await;

    // then
    assert!(result.is_err());
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("user")))]
pub async fn update_returns_a_user_when_user_id_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given 
    let user_id = 1;   
    let request = UpdateUser {
        first_name: "John".to_string(),
        middle_name: Some("Pope".to_string()),
        surname: "Doe".to_string(),
        mobile_number: Some("0700000000".to_string()),
    };

    // when
    let result = db.users.update(&user_id, &request).await;

    // then
    assert!(result.is_ok());

    let update_user = result.unwrap();

    assert_eq!(update_user.user_id, user_id);
    assert_eq!(update_user.first_name, request.first_name);
    assert_eq!(update_user.middle_name.unwrap(), "Pope");
    assert_eq!(update_user.surname, request.surname);
    assert_eq!(update_user.email_address, "jsmith@test.com");
    assert_eq!(update_user.mobile_number.unwrap(), "0700000000");
}

#[sqlx::test]
pub async fn update_return_error_when_user_id_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given 
    let user_id = 1;   
    let request = UpdateUser {
        first_name: "John".to_string(),
        middle_name: Some("Pope".to_string()),
        surname: "Doe".to_string(),
        mobile_number: Some("0700000000".to_string()),
    };

    // when
    let result = db.users.update(&user_id, &request).await;

    // then
    assert!(result.is_err());
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("user", "user_credential")))]
pub async fn delete_by_id_returns_rows_affected_eq_one_when_id_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let user_id = 1;

    // when
    let result = db.users.delete(&user_id).await;
    
    // then
    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result.rows_affected(),  1);
}

#[sqlx::test]
pub async fn delete_by_id_returns_rows_affected_eq_zero_when_id_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let user_id = 1;

    // when
    let result = db.users.delete(&user_id).await;
    
    // then
    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result.rows_affected(),  0);
}
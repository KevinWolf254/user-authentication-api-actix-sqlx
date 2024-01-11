use bulk_sms_api::{dao::Database, entity::role::{Role, CreateRole}};
use chrono::{DateTime, Utc};
use sqlx::Pool;

#[sqlx::test]
pub async fn find_by_id_returns_role_when_id_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let name = "ROLE_WRITE".to_string();
    let now: DateTime<Utc> = Utc::now();

    let created_role = sqlx::query_as!(Role, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."ROLE" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.roles.pool)
        .await.unwrap();

    // when
    let result = db.roles.find_by_id(&created_role.role_id).await;
    
    // then
    assert!(result.is_ok());

    let created_role = result.unwrap();

    assert!(created_role.role_id.is_positive());
    assert_eq!(created_role.name, name);
}

#[sqlx::test]
pub async fn find_by_id_returns_error_when_id_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let role_id = 2001;

    // when
    let result = db.roles.find_by_id(&role_id).await;

    // then
    assert!(result.is_err());
}

#[sqlx::test]
pub async fn find_all_returns_roles_when_roles_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let mut name = "PERMISSION_READ".to_string();
    let mut now: DateTime<Utc> = Utc::now();

    sqlx::query_as!(Role, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."ROLE" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.roles.pool)
        .await.unwrap();

    name = "Role_WRITE".to_string();
    now = Utc::now();

    sqlx::query_as!(Role, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."ROLE" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.roles.pool)
        .await.unwrap();

    // when
    let result = db.roles.find_all().await;

    // then
    assert!(result.is_ok());

    let roles = result.unwrap();

    assert_eq!(roles.len(), 2);
}

#[sqlx::test]
pub async fn find_all_returns_empty_when_roles_do_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // when
    let result = db.roles.find_all().await;

    // then
    assert!(result.is_ok());

    let roles = result.unwrap();

    assert_eq!(roles.len(), 0);
}

#[sqlx::test]
pub async fn find_paginated_returns_paginated_result_when_roles_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let page = 1;
    let page_size = 5;

    let mut name = "ROLE_READ".to_string();
    let mut now: DateTime<Utc> = Utc::now();

    let saved = sqlx::query_as!(Role, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."ROLE" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.roles.pool)
        .await.unwrap();

    dbg!("{:?}", saved);

    name = "ROLE_WRITE".to_string();
    now = Utc::now();

    sqlx::query_as!(Role, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."ROLE" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.roles.pool)
        .await.unwrap();

    // when
    let result = db.roles.find_paginated(page, page_size).await;

    // then
    assert!(result.is_ok());

    let result = result.unwrap();

    dbg!("{:?}", &result);

    assert_eq!(result.data.len(), 2);
    assert_eq!(result.total, 2);
    assert_eq!(result.page, page);
    assert_eq!(result.page_size, 5);

}

#[sqlx::test]
pub async fn create_returns_a_role_when_role_name_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let name = "ROLE_READ".to_string();
    
    let role = CreateRole {
        name: name.clone()
    };

    // when
    let result = db.roles.create(&role).await;

    // then
    assert!(result.is_ok());

    let created_role= result.unwrap();

    assert!(created_role.role_id.is_positive());
    assert_eq!(created_role.name, name);
}

#[sqlx::test]
pub async fn create_returns_an_error_when_role_name_already_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let name = "ROLE_UPDATE".to_string();
    let now: DateTime<Utc> = Utc::now();

    sqlx::query_as!(role, 
        r#"DELETE FROM "SMS_GATEWAY_USER"."ROLE" WHERE name = $1 "#, &name)
        .execute(&*db.roles.pool)
        .await.unwrap();
    
    sqlx::query_as!(Role, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."ROLE" (name, created_at) VALUES ($1, $2) RETURNING * "#, &name, &now)
        .fetch_one(&*db.roles.pool)
        .await.unwrap();
    
    let role = CreateRole {
        name: name.clone()
    };

    // when
    let result = db.roles.create(&role).await;

    // then
    assert!(result.is_err());
}

#[sqlx::test]
pub async fn delete_by_id_returns_rows_affected_eq_one_when_id_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let name = "ROLE_WRITE".to_string();
    let now: DateTime<Utc> = Utc::now();

    let created_role = sqlx::query_as!(Role, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."ROLE" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.roles.pool)
        .await.unwrap();

    // when
    let result = db.roles.delete(&created_role.role_id).await;
    
    // then
    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result.rows_affected(),  1);
}

#[sqlx::test]
pub async fn delete_by_id_returns_rows_affected_eq_zero_when_id_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let role_id = 2001;

    // when
    let result = db.roles.delete(&role_id).await;
    
    // then
    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result.rows_affected(),  0);
}
use bulk_sms_api::{entity::permission::{CreatePermission, Permission}, dao::Database};
use chrono::{DateTime, Utc};
use sqlx::Pool;

#[sqlx::test]
pub async fn find_by_id_returns_permission_when_id_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let name = "PERMISSION_WRITE".to_string();
    let now: DateTime<Utc> = Utc::now();

    let created_permission = sqlx::query_as!(Permission, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."PERMISSION" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.permissions.pool)
        .await.unwrap();

    // when
    let result = db.permissions.find_by_id(&created_permission.permission_id).await;
    
    // then
    assert!(result.is_ok());

    let created_permission = result.unwrap();

    assert!(created_permission.permission_id.is_positive());
    assert_eq!(created_permission.name, name);
}


#[sqlx::test]
pub async fn find_by_id_returns_error_when_id_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let permission_id = 2001;

    // when
    let result = db.permissions.find_by_id(&permission_id).await;

    // then
    assert!(result.is_err());
}

#[sqlx::test]
pub async fn find_all_returns_permissions_when_permissions_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let mut name = "PERMISSION_READ".to_string();
    let mut now: DateTime<Utc> = Utc::now();

    sqlx::query_as!(Permission, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."PERMISSION" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.permissions.pool)
        .await.unwrap();

    name = "PERMISSION_WRITE".to_string();
    now = Utc::now();

    sqlx::query_as!(Permission, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."PERMISSION" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.permissions.pool)
        .await.unwrap();

    // when
    let result = db.permissions.find_all().await;

    // then
    assert!(result.is_ok());

    let permissions = result.unwrap();

    assert!(permissions.len() == 2);
}

#[sqlx::test]
pub async fn find_all_returns_empty_when_permissions_do_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // when
    let result = db.permissions.find_all().await;

    // then
    assert!(result.is_ok());

    let permissions = result.unwrap();

    assert_eq!(permissions.len(), 0);
}

#[sqlx::test]
pub async fn find_paginated_returns_paginated_result_when_permissions_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let page = 1;
    let page_size = 5;

    let mut name = "PERMISSION_TEST".to_string();
    let mut now: DateTime<Utc> = Utc::now();

    let saved = sqlx::query_as!(Permission, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."PERMISSION" (name, created_at) VALUES ($1, $2) RETURNING permission_id, name, created_at "#, name, now)
        .fetch_one(&*db.permissions.pool)
        .await.unwrap();

    dbg!("{:?}", saved);

    name = "PERMISSION_WRITE".to_string();
    now = Utc::now();

    sqlx::query_as!(Permission, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."PERMISSION" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.permissions.pool)
        .await.unwrap();

    // when
    let result = db.permissions.find_paginated(page, page_size).await;

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
pub async fn create_returns_a_permission_when_permission_name_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let name = "PERMISSION_READ".to_string();
    
    let permission = CreatePermission {
        name: name.clone()
    };

    // when
    let result = db.permissions.create(&permission).await;

    // then
    assert!(result.is_ok());

    let created_permission = result.unwrap();

    assert!(created_permission.permission_id.is_positive());
    assert_eq!(created_permission.name, name);
}

#[sqlx::test]
pub async fn create_returns_an_error_when_permission_name_already_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let name = "PERMISSION_UPDATE".to_string();
    let now: DateTime<Utc> = Utc::now();

    sqlx::query_as!(Permission, 
        r#"DELETE FROM "SMS_GATEWAY_USER"."PERMISSION" WHERE name = $1 "#, &name)
        .execute(&*db.permissions.pool)
        .await.unwrap();
    
    sqlx::query_as!(Permission, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."PERMISSION" (name, created_at) VALUES ($1, $2) RETURNING * "#, &name, &now)
        .fetch_one(&*db.permissions.pool)
        .await.unwrap();
    
    let permission = CreatePermission {
        name: name.clone()
    };

    // when
    let result = db.permissions.create(&permission).await;

    // then
    assert!(result.is_err());
}

#[sqlx::test]
pub async fn delete_by_id_returns_rows_affected_eq_one_when_id_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let name = "PERMISSION_WRITE".to_string();
    let now: DateTime<Utc> = Utc::now();

    let created_permission = sqlx::query_as!(Permission, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."PERMISSION" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.permissions.pool)
        .await.unwrap();

    // when
    let result = db.permissions.delete(&created_permission.permission_id).await;
    
    // then
    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result.rows_affected(),  1);
}

#[sqlx::test]
pub async fn delete_by_id_returns_rows_affected_eq_zero_when_id_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let permission_id = 2001;

    // when
    let result = db.permissions.delete(&permission_id).await;
    
    // then
    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result.rows_affected(),  0);
}
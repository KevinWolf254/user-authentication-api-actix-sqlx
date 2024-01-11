use bulk_sms_api::{entity::{role::Role, permission::Permission}, dao::db_context::Database};
use chrono::{DateTime, Utc};
use sqlx::Pool;


#[sqlx::test]
pub async fn create_returns_rows_affected_ne_zero(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let name = "ADMIN".to_string();
    let now: DateTime<Utc> = Utc::now();

    let role = sqlx::query_as!(Role, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."ROLE" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.roles.pool)
        .await.unwrap();
    
    let name = "PERMISSION_READ".to_string();

    let permission = sqlx::query_as!(Permission, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."PERMISSION" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.permissions.pool)
        .await.unwrap();

    let name = "PERMISSION_WRITE".to_string();

    let permission_two = sqlx::query_as!(Permission, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."PERMISSION" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.permissions.pool)
        .await.unwrap();

    // when
    let result = db.role_permissions.create_role_permissions(&role.role_id, &vec![permission, permission_two]).await.unwrap();

    // then
    assert_eq!(result, 2);

    // when
    let result = db.role_permissions.create_role_permissions(&role.role_id, &vec![]).await.unwrap();

    // then
    assert_eq!(result, 0);
}

#[sqlx::test]
pub async fn create_returns_error_when_role_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let name = "PERMISSION_READ".to_string();
    let now: DateTime<Utc> = Utc::now();

    let permission = sqlx::query_as!(Permission, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."PERMISSION" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.permissions.pool)
        .await.unwrap();

    let name = "PERMISSION_WRITE".to_string();

    let permission_two = sqlx::query_as!(Permission, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."PERMISSION" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.permissions.pool)
        .await.unwrap();

    // when
    let result = db.role_permissions.create_role_permissions(&2000, &vec![permission, permission_two]).await;

    // then
    assert!(result.is_err());
}

#[sqlx::test]
pub async fn find_role_permissions_returns_permissions_when_role_id_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;
    
    // given
    let name = "ADMIN".to_string();
    let now: DateTime<Utc> = Utc::now();

    let role = sqlx::query_as!(Role, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."ROLE" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.roles.pool)
        .await.unwrap();
    
    let name = "PERMISSION_READ".to_string();

    let permission = sqlx::query_as!(Permission, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."PERMISSION" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.permissions.pool)
        .await.unwrap();

    let name = "PERMISSION_WRITE".to_string();

    let permission_two = sqlx::query_as!(Permission, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."PERMISSION" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.permissions.pool)
        .await.unwrap();

    sqlx::query!( 
        r#"INSERT INTO "SMS_GATEWAY_USER"."ROLE_PERMISSION" (role_id, permission_id) VALUES ($1, $2) "#, &role.role_id, &permission.permission_id)
        .execute(&*db.permissions.pool)
        .await.unwrap();

    sqlx::query!( 
        r#"INSERT INTO "SMS_GATEWAY_USER"."ROLE_PERMISSION" (role_id, permission_id) VALUES ($1, $2) "#, &role.role_id, &permission_two.permission_id)
        .execute(&*db.permissions.pool)
        .await.unwrap();
    // when
    let result = db.role_permissions.find_role_permissions(&role.role_id).await.unwrap();

    // then
    assert_eq!(result.len(), 2);

}

#[sqlx::test]
pub async fn find_role_permissions_returns_empty_when_role_id_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;
    
    // given
    let role_id = 20001;

    // when
    let result = db.role_permissions.find_role_permissions(&role_id).await;

    dbg!("{:?}", &result);
    // then
    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result.len(), 0);
}

#[sqlx::test]
pub async fn update_role_permissions_returns_ne_zero_when_updated(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let name = "ADMIN".to_string();
    let now: DateTime<Utc> = Utc::now();

    let role = sqlx::query_as!(Role, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."ROLE" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.roles.pool)
        .await.unwrap();

    let name = "PERMISSION_READ".to_string();

    let permission = sqlx::query_as!(Permission, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."PERMISSION" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.permissions.pool)
        .await.unwrap();

    let name = "PERMISSION_WRITE".to_string();

    let permission_two = sqlx::query_as!(Permission, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."PERMISSION" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.permissions.pool)
        .await.unwrap();

    sqlx::query!( 
        r#"INSERT INTO "SMS_GATEWAY_USER"."ROLE_PERMISSION" (role_id, permission_id) VALUES ($1, $2) "#, &role.role_id, &permission.permission_id)
        .execute(&*db.permissions.pool)
        .await.unwrap();

    let permissions = sqlx::query_as!(Permission, 
        r#"	SELECT * FROM "SMS_GATEWAY_USER"."PERMISSION" p WHERE p.permission_id IN (SELECT r.permission_id FROM "SMS_GATEWAY_USER"."ROLE_PERMISSION" r WHERE r.role_id = $1)"#, 
        &role.role_id)
        .fetch_all(&*db.role_permissions.pool)
        .await.unwrap();

    assert_eq!(permissions.len(), 1);

    // when
    let result = db.role_permissions.update_role_permissions(&role.role_id, &vec![permission_two]).await.unwrap();

    // then
    assert_eq!(result, 2);

    let result = db.role_permissions.update_role_permissions(&role.role_id, &vec![]).await.unwrap();

    // then
    assert_eq!(result, 1);
}

#[sqlx::test]
pub async fn update_role_permissions_returns_eq_zero_when_not_updated(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let name = "ADMIN".to_string();
    let now: DateTime<Utc> = Utc::now();

    let role = sqlx::query_as!(Role, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."ROLE" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.roles.pool)
        .await.unwrap();

    let name = "PERMISSION_READ".to_string();

    let permission = sqlx::query_as!(Permission, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."PERMISSION" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.permissions.pool)
        .await.unwrap();

    sqlx::query!( 
        r#"INSERT INTO "SMS_GATEWAY_USER"."ROLE_PERMISSION" (role_id, permission_id) VALUES ($1, $2) "#, &role.role_id, &permission.permission_id)
        .execute(&*db.permissions.pool)
        .await.unwrap();

    let permissions = sqlx::query_as!(Permission, 
        r#"	SELECT * FROM "SMS_GATEWAY_USER"."PERMISSION" p WHERE p.permission_id IN (SELECT r.permission_id FROM "SMS_GATEWAY_USER"."ROLE_PERMISSION" r WHERE r.role_id = $1)"#, 
        &role.role_id)
        .fetch_all(&*db.role_permissions.pool)
        .await.unwrap();

    assert_eq!(permissions.len(), 1);

    // when
    let result = db.role_permissions.update_role_permissions(&role.role_id, &vec![permission]).await.unwrap();

    // then
    assert_eq!(result, 2);

}

#[sqlx::test]
pub async fn delete_role_permissions_returns_ne_zero_when_role_id_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let name = "ADMIN".to_string();
    let now: DateTime<Utc> = Utc::now();

    let role = sqlx::query_as!(Role, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."ROLE" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.roles.pool)
        .await.unwrap();

    let name = "PERMISSION_READ".to_string();

    let permission = sqlx::query_as!(Permission, 
        r#"INSERT INTO "SMS_GATEWAY_USER"."PERMISSION" (name, created_at) VALUES ($1, $2) RETURNING * "#, name, now)
        .fetch_one(&*db.permissions.pool)
        .await.unwrap();

    sqlx::query!( 
        r#"INSERT INTO "SMS_GATEWAY_USER"."ROLE_PERMISSION" (role_id, permission_id) VALUES ($1, $2) "#, &role.role_id, &permission.permission_id)
        .execute(&*db.permissions.pool)
        .await.unwrap();
    // when
    let result = db.role_permissions.delete_role_permissions(&role.role_id).await;

    // then
    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result, 1);
}

#[sqlx::test]
pub async fn delete_role_permissions_returns_eq_zero_when_role_id_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;
    // given
    let role_id = 2000;
    // when
    let result = db.role_permissions.delete_role_permissions(&role_id).await;

    // then
    assert!(result.is_ok());

    let result = result.unwrap();
    assert_eq!(result, 0);
}
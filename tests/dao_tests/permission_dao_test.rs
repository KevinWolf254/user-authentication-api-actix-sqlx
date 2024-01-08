use bulk_sms_api::entity::permission::{CreatePermission, Permission};
use chrono::{DateTime, Utc};

use crate::init_test_db;

#[actix_rt::test]
pub async fn create_returns_a_permission_when_successful() {
    let db = init_test_db().await;

    // given
    let name = "PERMISSION_READ".to_string();

    sqlx::query_as!(Permission, 
        r#"DELETE FROM "SMS_GATEWAY_USER"."PERMISSION" WHERE name = $1 "#, &name)
        .execute(&*db.permissions.pool)
        .await.unwrap();
    
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

#[actix_rt::test]
pub async fn create_returns_an_error_when_already_exists() {
    let db = init_test_db().await;

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

#[actix_rt::test]
pub async fn find_by_id_returns_permission_when_successful() {
    let db = init_test_db().await;
    
    // given
    let name = "PERMISSION_WRITE".to_string();
    let now: DateTime<Utc> = Utc::now();

    sqlx::query_as!(Permission, 
        r#"DELETE FROM "SMS_GATEWAY_USER"."PERMISSION" WHERE name = $1 "#, &name)
        .execute(&*db.permissions.pool)
        .await.unwrap();

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

#[actix_rt::test]
pub async fn find_by_id_returns_not_found_when_unsuccessful() {
    let db = init_test_db().await;

    // given
    let permission_id = 2001;

    // when
    let result = db.permissions.find_by_id(&permission_id).await;

    // then
    assert!(result.is_err());
}
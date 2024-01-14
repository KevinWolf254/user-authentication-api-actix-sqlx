use bulk_sms_api::dao::db_context::Database;
use sqlx::Pool;


#[sqlx::test(fixtures(path = "../fixtures", scripts("role", "permission")))]
pub async fn create_returns_rows_affected_ne_zero(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let role_id = 1;
    let permissions = db.permissions.find_all().await.unwrap();

    // when
    let result = db.role_permissions.create_role_permissions(&role_id, &permissions).await.unwrap();

    // then
    assert_eq!(result, 4);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("permission")))]
pub async fn create_returns_error_when_role_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let permissions = db.permissions.find_all().await.unwrap();

    // when
    let result = db.role_permissions.create_role_permissions(&2000, &permissions).await;

    // then
    assert!(result.is_err());
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("role", "permission", "role_permission")))]
pub async fn find_role_permissions_returns_permissions_when_role_has_permissions(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;
    
    // given
    let role_id = 1;
    
    // when
    let result = db.role_permissions.find_role_permissions(&role_id).await.unwrap();

    // then
    assert_eq!(result.len(), 4);

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
    assert!(result.is_err());
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("role", "permission", "role_permission")))]
pub async fn update_role_permissions_returns_ne_zero_when_updated(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let role_id = 1;

    // when
    let result = db.role_permissions.update_role_permissions(&role_id, &vec![]).await.unwrap();

    // then
    assert_eq!(result, 4);
    let permissions = db.role_permissions.find_role_permissions(&role_id).await.unwrap();

    assert_eq!(permissions.len(), 0);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("role")))]
pub async fn update_role_permissions_returns_eq_zero_when_not_updated(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let role_id = 1;

    // when
    let result = db.role_permissions.update_role_permissions(&role_id, &vec![]).await.unwrap();

    // then
    assert_eq!(result, 0);

}

#[sqlx::test(fixtures(path = "../fixtures", scripts("role", "permission", "role_permission")))]
pub async fn delete_role_permissions_returns_ne_zero_when_role_id_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let role_id = 1;
    
    // when
    let result = db.role_permissions.delete_role_permissions(&role_id).await;

    // then
    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result, 4);
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
use bulk_sms_api::dao::db_context::Database;
use sqlx::Pool;

#[sqlx::test(fixtures(path = "../fixtures", scripts("user", "role")))]
pub async fn create_returns_rows_affected_gt_zero(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let user_id = 1;
    let roles = db.roles.find_all().await.unwrap();

    // when
    let result = db.user_role.create_user_roles(&user_id, &roles).await.unwrap();

    // then
    assert_eq!(result, 4);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("role")))]
pub async fn create_returns_error_when_user_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let roles = db.roles.find_all().await.unwrap();

    // when
    let result = db.user_role.create_user_roles(&2000, &roles).await;

    // then
    assert!(result.is_err());
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("user", "role", "user_role")))]
pub async fn find_user_role_returns_roles(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;
    
    // given
    let user_id = 1;
    
    // when
    let result = db.user_role.find_user_roles(&user_id).await.unwrap();

    // then
    assert_eq!(result.len(), 1);

}

#[sqlx::test]
pub async fn find_user_role_returns_empty_when_user_id_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;
    
    // given
    let user_id = 20001;

    // when
    let result = db.user_role.find_user_roles(&user_id).await;

    dbg!("{:?}", &result);
    // then
    assert!(result.is_err());
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("user", "role", "user_role")))]
pub async fn update_user_role_returns_gt_zero_when_updated(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let user_id = 1;

    // when
    let result = db.user_role.update_user_roles(&user_id, &vec![]).await.unwrap();

    // then
    assert_eq!(result, 1);
    let roles = db.user_role.find_user_roles(&user_id).await.unwrap();

    assert_eq!(roles.len(), 0);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("user")))]
pub async fn update_user_role_returns_eq_zero_when_not_updated(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let user_id = 1;

    // when
    let result = db.user_role.update_user_roles(&user_id, &vec![]).await.unwrap();

    // then
    assert_eq!(result, 0);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("user", "role", "user_role")))]
pub async fn delete_user_role_returns_gt_zero_when_user_id_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let user_id = 1;
    
    // when
    let result = db.user_role.delete_user_roles(&user_id).await;

    // then
    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result, 1);
}

#[sqlx::test]
pub async fn delete_user_role_returns_eq_zero_when_user_id_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;
    // given
    let user_id = 2000;
    
    // when
    let result = db.user_role.delete_user_roles(&user_id).await;

    // then
    assert!(result.is_ok());

    let result = result.unwrap();
    assert_eq!(result, 0);
}
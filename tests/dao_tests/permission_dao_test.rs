use bulk_sms_api::{entity::permission::CreatePermission, dao::Database};
use sqlx::Pool;

// #[sqlx::test]
#[sqlx::test(fixtures(path = "../fixtures", scripts("permission")))]
pub async fn find_by_id_returns_permission_when_id_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    let permission_id = 1;
    // when
    let result = db.permissions.find_by_id(&permission_id).await;
    
    // then
    assert!(result.is_ok());

    let created_permission = result.unwrap();

    assert!(created_permission.permission_id.is_positive());
    assert_eq!(created_permission.name, "PERMISSION_READ");
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

#[sqlx::test(fixtures(path = "../fixtures", scripts("permission")))]
pub async fn find_all_returns_permissions_when_permissions_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    // when
    let result = db.permissions.find_all().await;

    // then
    assert!(result.is_ok());

    let permissions = result.unwrap();

    assert_eq!(permissions.len(), 4);
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

#[sqlx::test(fixtures(path = "../fixtures", scripts("permission")))]
pub async fn find_paginated_returns_paginated_result_when_permissions_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let page = 1;
    let page_size = 5;

    // when
    let result = db.permissions.find_paginated(page, page_size).await;

    // then
    assert!(result.is_ok());

    let result = result.unwrap();

    dbg!("{:?}", &result);

    assert_eq!(result.data.len(), 4);
    assert_eq!(result.total, 4);
    assert_eq!(result.page, page);
    assert_eq!(result.page_size, page_size);

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

#[sqlx::test(fixtures(path = "../fixtures", scripts("permission")))]
pub async fn create_returns_an_error_when_permission_name_already_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let name = "PERMISSION_READ";
    
    let permission = CreatePermission {
        name: name.to_string()
    };

    // when
    let result = db.permissions.create(&permission).await;

    // then
    assert!(result.is_err());
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("permission")))]
pub async fn delete_by_id_returns_rows_affected_eq_one_when_id_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let permission_id = 1;

    // when
    let result = db.permissions.delete(&permission_id).await;
    
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
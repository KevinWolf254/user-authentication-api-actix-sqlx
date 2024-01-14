use bulk_sms_api::{dao::Database, entity::role::CreateRole};
use sqlx::Pool;

#[sqlx::test(fixtures(path = "../fixtures", scripts("role")))]
pub async fn find_by_id_returns_role_when_id_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let role_id = 1;

    // when
    let result = db.roles.find_by_id(&role_id).await;
    
    // then
    assert!(result.is_ok());

    let created_role = result.unwrap();

    assert!(created_role.role_id.is_positive());
    assert_eq!(created_role.name, "SUPER_ADMIN");
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

#[sqlx::test(fixtures(path = "../fixtures", scripts("role")))]
pub async fn find_all_returns_roles_when_roles_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    // when
    let result = db.roles.find_all().await;

    // then
    assert!(result.is_ok());

    let roles = result.unwrap();

    assert_eq!(roles.len(), 4);
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

#[sqlx::test(fixtures(path = "../fixtures", scripts("role")))]
pub async fn find_paginated_returns_paginated_result_when_roles_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let page = 1;
    let page_size = 5;

    // when
    let result = db.roles.find_paginated(page, page_size).await;

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
pub async fn create_returns_a_role_when_role_name_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let name = "SUPER_ADMIN";
    
    let role = CreateRole {
        name: name.to_string()
    };

    // when
    let result = db.roles.create(&role).await;

    // then
    assert!(result.is_ok());

    let created_role= result.unwrap();

    assert!(created_role.role_id.is_positive());
    assert_eq!(created_role.name, name);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("role")))]
pub async fn create_returns_an_error_when_role_name_already_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let name = "SUPER_ADMIN";
    
    let role = CreateRole {
        name: name.to_string()
    };

    // when
    let result = db.roles.create(&role).await;

    // then
    assert!(result.is_err());
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("role")))]
pub async fn delete_by_id_returns_rows_affected_eq_one_when_id_exists(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let role_id = 1;

    // when
    let result = db.roles.delete(&role_id).await;
    
    // then
    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result.rows_affected(),  1);
}

#[sqlx::test]
pub async fn delete_by_id_returns_rows_affected_eq_zero_when_id_does_not_exist(pool: Pool<sqlx::Postgres>) {
    let db = Database::test(pool).await;

    // given
    let role_id = 20001;

    // when
    let result = db.roles.delete(&role_id).await;
    
    // then
    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result.rows_affected(),  0);
}
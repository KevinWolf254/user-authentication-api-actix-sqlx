use sqlx::postgres::PgQueryResult;

use crate::entity::{user::User, user_code::UserCode};

use super::Table;

impl<'c> Table<'c, UserCode> {

    pub async fn create(&self, user_id: &i32, code: &i32) -> Result<UserCode, sqlx::Error> {

        sqlx::query_as!(User, 
            r#"SELECT * FROM "SMS_GATEWAY_USER"."USER" WHERE user_id = $1 "#, user_id)
            .fetch_one(&*self.pool)
            .await?;

        sqlx::query_as!(UserCode, 
            r#"INSERT INTO "SMS_GATEWAY_USER"."USER_CODE" (code, user_id) VALUES ($1, $2) RETURNING * "#, 
            code, user_id)
            .fetch_one(&*self.pool) 
            .await
    }

    pub async fn update(&self, user_id: &i32, code: &i32) -> Result<UserCode, sqlx::Error> {

        sqlx::query_as!(UserCode, 
            r#"UPDATE "SMS_GATEWAY_USER"."USER_CODE" SET code = $1 WHERE  user_id = $2 RETURNING * "#, 
            code, user_id)
            .fetch_one(&*self.pool) 
            .await
    }

    pub async fn find_by_user_id(&self, user_id: &i32) -> Result<UserCode, sqlx::Error> {
        sqlx::query_as!(UserCode, 
            r#"SELECT * FROM "SMS_GATEWAY_USER"."USER_CODE" WHERE user_id = $1 "#, user_id)
            .fetch_one(&*self.pool)
            .await
    }

    pub async fn find_by_user_id_and_code(&self, user_id: &i32, code: &i32) -> Result<UserCode, sqlx::Error> {
        sqlx::query_as!(UserCode, 
            r#"SELECT * FROM "SMS_GATEWAY_USER"."USER_CODE" WHERE user_id = $1 and code = $2"#, user_id, code)
            .fetch_one(&*self.pool)
            .await
    }

    pub async fn delete(&self, user_id: &i32, code: &i32) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query_as!(PgQueryResult, 
            r#"DELETE FROM "SMS_GATEWAY_USER"."USER_CODE" WHERE user_id = $1 and code = $2"#, user_id, code)
            .execute(&*self.pool)
            .await
    }
}
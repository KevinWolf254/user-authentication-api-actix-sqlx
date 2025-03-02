use crate::{entity::user_credential::UserCredential, model::user_credentials::{CreateUserCredential, UpdateUserCredential}};

use super::Table;

impl<'c> Table<'c, UserCredential> {

    pub async fn create(&self, user_id: &i32, request: &CreateUserCredential) -> Result<UserCredential, sqlx::Error> {
        let CreateUserCredential { username, password } = request;

        sqlx::query_as!(UserCredential, 
            r#"INSERT INTO "SMS_GATEWAY_USER"."USER_CREDENTIAL" (username, password, user_id) VALUES ($1, $2, $3) RETURNING * "#, 
            username, password, user_id)
            .fetch_one(&*self.pool) 
            .await
    }

    pub async fn update(&self, user_id: &i32, user_credential_id: &i32, request: &UpdateUserCredential) -> Result<UserCredential, sqlx::Error> {
        let UpdateUserCredential { previous_password: _, password } = request;

        sqlx::query_as!(UserCredential, 
            r#"UPDATE "SMS_GATEWAY_USER"."USER_CREDENTIAL" SET password = $1 WHERE user_credential_id = $2 AND user_id = $3 RETURNING * "#, 
            password, user_credential_id, user_id)
            .fetch_one(&*self.pool) 
            .await
    }

    pub async fn find_by_user_id(&self, user_id: &i32) -> Result<UserCredential, sqlx::Error> {
        sqlx::query_as!(UserCredential, 
            r#"SELECT * FROM "SMS_GATEWAY_USER"."USER_CREDENTIAL" WHERE user_id = $1 "#, user_id)
            .fetch_one(&*self.pool)
            .await
    }
}
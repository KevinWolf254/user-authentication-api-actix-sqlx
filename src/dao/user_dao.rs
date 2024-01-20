use sqlx::postgres::PgQueryResult;

use crate::{entity::user::User, dto::{pagination::PaginatedResult, user::{CreateUser, UpdateUser}}};

use super::{Table, CountResult};

impl<'c> Table<'c, User> {

    pub async fn find_by_id(&self, user_id: &i32) -> Result<User, sqlx::Error> {
        sqlx::query_as!(User, 
            r#"SELECT * FROM "SMS_GATEWAY_USER"."USER" WHERE user_id = $1 "#, user_id)
            .fetch_one(&*self.pool)
            .await
    }

    pub async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as!(User, r#"SELECT * FROM "SMS_GATEWAY_USER"."USER" "#)
            .fetch_all(&*self.pool)
            .await
    }

    pub async fn find_paginated(&self, page: i64, page_size: i64) -> Result<PaginatedResult<User>, sqlx::Error> {
        let users = self.find_by_page_and_page_size(&page, &page_size).await?;
        let total = self.find_total().await?;
    
        let result = PaginatedResult {
            data: users,
            total: total.count.unwrap_or(0),
            page,
            page_size
        };
    
        Ok(result)
    }

    async fn find_by_page_and_page_size(&self, page: &i64, page_size: &i64) -> Result<Vec<User>, sqlx::Error> {
        let offset = (page - 1) * page_size;
        sqlx::query_as!(User, r#"SELECT * FROM "SMS_GATEWAY_USER"."USER" ORDER BY USER_id DESC LIMIT $1 OFFSET $2"#, page_size, offset)
            .fetch_all(&*self.pool)
            .await
    }

    async fn find_total(&self) -> Result<CountResult, sqlx::Error>  {
        sqlx::query_as!(CountResult, 
            r#"SELECT COUNT(*) FROM "SMS_GATEWAY_USER"."USER""#)
            .fetch_one(&*self.pool)
            .await        
    }

    pub async fn create(&self, request: &CreateUser) -> Result<User, sqlx::Error> {
        let CreateUser { first_name, middle_name, surname, email_address, mobile_number } = request;

        sqlx::query_as!(User, 
            r#"INSERT INTO "SMS_GATEWAY_USER"."USER" (first_name, middle_name, surname, email_address, mobile_number) VALUES ($1, $2, $3, $4, $5) RETURNING * "#, 
            first_name, *middle_name, surname, email_address, *mobile_number)
            .fetch_one(&*self.pool) 
            .await
    }

    pub async fn update(&self, user_id: &i32, request: &UpdateUser) -> Result<User, sqlx::Error> {
        self.find_by_id(user_id).await?;

        let UpdateUser { first_name, middle_name, surname, mobile_number } = request;

        sqlx::query_as!(User, 
            r#"UPDATE "SMS_GATEWAY_USER"."USER" SET first_name = $1, middle_name = $2, surname = $3, mobile_number = $4 WHERE user_id = $5 RETURNING * "#, 
            first_name, *middle_name, surname, *mobile_number, user_id)
            .fetch_one(&*self.pool) 
            .await
    }

    pub async fn delete(&self, user_id: &i32) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query_as!(PgQueryResult, 
            r#"DELETE FROM "SMS_GATEWAY_USER"."USER" WHERE user_id = $1 "#, user_id)
            .execute(&*self.pool)
            .await
    }
}

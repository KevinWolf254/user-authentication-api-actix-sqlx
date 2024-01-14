use sqlx::postgres::PgQueryResult;

use crate::{entity::role::{Role, CreateRole}, dto::pagination::PaginatedResult};

use super::{Table, CountResult};

impl<'c> Table<'c, Role> {

    pub async fn find_by_id(&self, role_id: &i16) -> Result<Role, sqlx::Error> {
        sqlx::query_as!(Role, 
            r#"SELECT * FROM "SMS_GATEWAY_USER"."ROLE" WHERE role_id = $1 "#, role_id)
            .fetch_one(&*self.pool)
            .await
    }

    pub async fn find_all(&self) -> Result<Vec<Role>, sqlx::Error> {
        sqlx::query_as!(Role, r#"SELECT * FROM "SMS_GATEWAY_USER"."ROLE" "#)
            .fetch_all(&*self.pool)
            .await
    }

    pub async fn find_paginated(&self, page: i64, page_size: i64) -> Result<PaginatedResult<Role>, sqlx::Error> {
        let roles = self.find_by_page_and_page_size(&page, &page_size).await?;
        let total = self.find_total().await?;

        let result = PaginatedResult {
            data: roles,
            total: total.count.unwrap_or(0),
            page,
            page_size
        };

        Ok(result)
    }

    async fn find_by_page_and_page_size(&self, page: &i64, page_size: &i64) -> Result<Vec<Role>, sqlx::Error> {
        let offset = (page - 1) * page_size;
        sqlx::query_as!(Role, r#"SELECT * FROM "SMS_GATEWAY_USER"."ROLE" ORDER BY role_id DESC LIMIT $1 OFFSET $2"#, page_size, offset)
            .fetch_all(&*self.pool)
            .await
    }

    async fn find_total(&self) -> Result<CountResult, sqlx::Error>  {
        sqlx::query_as!(CountResult, 
            r#"SELECT COUNT(*) FROM "SMS_GATEWAY_USER"."ROLE""#)
            .fetch_one(&*self.pool)
            .await        
    }

    pub async fn create(&self, request: &CreateRole) -> Result<Role, sqlx::Error> {
        sqlx::query_as!(Role, 
            r#"INSERT INTO "SMS_GATEWAY_USER"."ROLE" (name) VALUES ($1) RETURNING * "#, request.name)
            .fetch_one(&*self.pool)
            .await
    }

    pub async fn delete(&self, role_id: &i16) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query_as!(PgQueryResult, 
            r#"DELETE FROM "SMS_GATEWAY_USER"."ROLE" WHERE role_id = $1 "#, role_id)
            .execute(&*self.pool)
            .await
    }
}
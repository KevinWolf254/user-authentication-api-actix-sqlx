use sqlx::postgres::PgQueryResult;

use crate::{entity::permission::{Permission, CreatePermission}, dto::pagination::PaginatedResult};

use super::{Table, CountResult};

impl<'c> Table<'c, Permission> {

    pub async fn find_by_id(&self, permission_id: &i16) -> Result<Permission, sqlx::Error> {
        sqlx::query_as!(Permission, 
            r#"SELECT * FROM "SMS_GATEWAY_USER"."PERMISSION" WHERE permission_id = $1 "#, permission_id)
            .fetch_one(&*self.pool)
            .await
    }

    pub async fn find_all(&self) -> Result<Vec<Permission>, sqlx::Error> {
        sqlx::query_as!(Permission, r#"SELECT * FROM "SMS_GATEWAY_USER"."PERMISSION" "#)
            .fetch_all(&*self.pool)
            .await
    }

    pub async fn find_paginated(&self, page: i64, page_size: i64) -> Result<PaginatedResult<Permission>, sqlx::Error> {
        let permissions = self.find_by_page_and_page_size(&page, &page_size).await?;
        let total = self.find_total().await?;

        let result = PaginatedResult {
            data: permissions,
            total: total.count.unwrap_or(0),
            page,
            page_size
        };

        Ok(result)
    }

    async fn find_by_page_and_page_size(&self, page: &i64, page_size: &i64) -> Result<Vec<Permission>, sqlx::Error> {
        let offset = (page - 1) * page_size;
        sqlx::query_as!(Permission, r#"SELECT * FROM "SMS_GATEWAY_USER"."PERMISSION" ORDER BY permission_id DESC LIMIT $1 OFFSET $2"#, page_size, offset)
            .fetch_all(&*self.pool)
            .await
    }

    async fn find_total(&self) -> Result<CountResult, sqlx::Error>  {
        sqlx::query_as!(CountResult, 
            r#"SELECT COUNT(*) FROM "SMS_GATEWAY_USER"."PERMISSION""#)
            .fetch_one(&*self.pool)
            .await        
    }

    pub async fn create(&self, request: &CreatePermission) -> Result<Permission, sqlx::Error> {
        sqlx::query_as!(Permission, 
            r#"INSERT INTO "SMS_GATEWAY_USER"."PERMISSION" (name) VALUES ($1) RETURNING * "#, request.name)
            .fetch_one(&*self.pool)
            .await
    }

    pub async fn delete(&self, permission_id: &i16) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query_as!(PgQueryResult, 
            r#"DELETE FROM "SMS_GATEWAY_USER"."PERMISSION" WHERE permission_id = $1 "#, permission_id)
            .execute(&*self.pool)
            .await
    }
}
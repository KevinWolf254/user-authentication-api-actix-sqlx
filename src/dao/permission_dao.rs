use std::borrow::Cow;

use chrono::{DateTime, Utc};
use sqlx::postgres::PgQueryResult;

use crate::{entity::permission::{Permission, CreatePermission}, error::AppError, error::AppErrorType};

use super::Table;
struct CountResult {
    pub count: Option<i64>
}
struct PaginatedResult<T> {
    data: Vec<T>,
    total: i64,
    page: i64,
    page_size: i64
}

impl<'c> Table<'c, Permission> {
    
    pub async fn find_by_id(&self, permission_id: &i16) -> Result<Permission, AppError> {
        sqlx::query_as!(Permission, 
            r#"SELECT * FROM "SMS_GATEWAY_USER"."PERMISSION" WHERE permission_id = $1 "#, permission_id)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| {
                match e {
                    sqlx::Error::RowNotFound => AppError::new(Some(format!("Permission with id {} could not be found!", permission_id)), None, AppErrorType::NotFoundError),
                    error  => {
                        AppError::new(None, Some(error.to_string()), AppErrorType::DBError)
                    }
                }
            })
    }

    pub async fn find_all(&self) -> Result<Vec<Permission>, AppError> {
        match
            sqlx::query_as!(Permission, r#"SELECT * FROM "SMS_GATEWAY_USER"."PERMISSION" "#)
            .fetch_all(&*self.pool)
            .await
        {
            Ok(permissons) => Ok(permissons),        
            Err(e) => { 
                match e {
                    sqlx::Error::RowNotFound => Ok(Vec::with_capacity(0)),
                    error  => {
                        Err(AppError::new(None, Some(error.to_string()), AppErrorType::DBError))
                    }
                }
            },
        }
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

    pub async fn create(&self, request: &CreatePermission) -> Result<Permission, AppError> {
        let now: DateTime<Utc> = Utc::now();
        sqlx::query_as!(Permission, 
            r#"INSERT INTO "SMS_GATEWAY_USER"."PERMISSION" (name, created_at) VALUES ($1, $2) RETURNING * "#, request.name, now)
            .fetch_one(&*self.pool)
            .await
            .map_err(|error| {
                match &error {
                    sqlx::Error::Database(d) => {
                        match d.code() {
                            Some(c) => {
                                if let Cow::Borrowed(code) = c {
                                    if code.eq("23505") {
                                        AppError::new(Some("Permission already exists!".to_string()), None, AppErrorType::BadRequestError)
                                    } else {
                                        AppError::new(None, Some(error.to_string()), AppErrorType::DBError)
                                    }
                                } else {
                                    AppError::new(None, Some(error.to_string()), AppErrorType::DBError)
                                }
                            },
                            None => AppError::new(None, Some(error.to_string()), AppErrorType::DBError),
                        }
                    },
                    _ => AppError::new(None, Some(error.to_string()), AppErrorType::DBError),
                }
            })

    }

    pub async fn delete(&self, permission_id: &i16) -> Result<PgQueryResult, AppError> {
        sqlx::query_as!(Permission, 
            r#"DELETE FROM "SMS_GATEWAY_USER"."PERMISSION" WHERE permission_id = $1 "#, permission_id)
            .execute(&*self.pool)
            .await
            .map_err(|error| AppError::new(None, Some(error.to_string()), AppErrorType::DBError))
    }
}
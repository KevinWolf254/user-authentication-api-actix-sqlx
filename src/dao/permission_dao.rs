use chrono::{DateTime, Utc};
use sqlx::postgres::PgQueryResult;

use crate::model::permission::{Permission, CreatePermission};

use super::Table;

impl<'c> Table<'c, Permission> {

    pub async fn find_by_id(&self, permission_id: i16) -> Result<Option<Permission>, sqlx::Error> {
        match
            sqlx::query_as!(Permission, r#"SELECT * FROM "SMS_GATEWAY_USER"."PERMISSION" WHERE permission_id = $1 "#, permission_id).fetch_one(&*self.pool).await
        {
            Ok(permisson) => Ok(Some(permisson)),        
            Err(e) => { 
                match e {
                    sqlx::Error::RowNotFound => Ok(None),
                    error  => {
                        println!("Error occured {:?}", error.to_string());
                        Err(error)
                    }
                }
            },
        }
    }

    pub async fn find_all(&self) -> Result<Vec<Permission>, sqlx::Error> {
        match
            sqlx::query_as!(Permission, r#"SELECT * FROM "SMS_GATEWAY_USER"."PERMISSION" "#).fetch_all(&*self.pool).await
        {
            Ok(permissons) => Ok(permissons),        
            Err(e) => { 
                match e {
                    sqlx::Error::RowNotFound => Ok(Vec::with_capacity(0)),
                    error  => {
                        println!("Error occured {:?}", error.to_string());
                        Err(error)
                    }
                }
            },
        }
    }

    pub async fn create(&self, request: CreatePermission) -> Result<Permission, sqlx::Error> {
        let now: DateTime<Utc> = Utc::now();
        sqlx::query_as!(Permission, r#"INSERT INTO "SMS_GATEWAY_USER"."PERMISSION" (name, created_at) VALUES ($1, $2) RETURNING * "#,
        &request.name, now).fetch_one(&*self.pool).await
    }

    pub async fn delete(&self, permission_id: i16) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query_as!(Permission, r#"DELETE FROM "SMS_GATEWAY_USER"."PERMISSION" WHERE permission_id = $1 "#, &permission_id).execute(&*self.pool).await
    }
}
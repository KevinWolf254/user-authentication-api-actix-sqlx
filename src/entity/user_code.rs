use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;
use sqlx::{FromRow, postgres::PgRow, Row};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserCode {
    pub user_code_id: i32,
    pub code: i32,
    pub created_at: DateTime<Utc>,
    pub user_id: i32,
}

impl<'c> FromRow<'c, PgRow> for UserCode {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(UserCode {
            user_code_id: row.get(0),
            code: row.get(1),
            created_at: row.get(2),
            user_id: row.get(3),
        })
    }
}
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use regex::Regex;
use validator::Validate;
use sqlx::{FromRow, postgres::PgRow, Row};

static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9A-Za-z_]+$").unwrap());

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserCredential {
    pub user_credential_id: i32,
    #[validate(length(min = 3, max = 16), regex = "USERNAME_REGEX")]
    pub username: String,
    pub password: String,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
}

impl<'c> FromRow<'c, PgRow> for UserCredential {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(UserCredential {
            user_credential_id: row.get(0),
            username: row.get(1),
            password: row.get(2),
            user_id: row.get(3),
            created_at: row.get(4),
        })
    }
}
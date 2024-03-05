use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, postgres::PgRow, Row};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: i32,
    #[validate(length(min = 3, message = "First name is required!"))]
    pub first_name: String,
    pub middle_name: Option<String>,
    #[validate(length(min = 3, message = "Surname is required!"))]
    pub surname: String,
    #[validate(email(message = "Email address is not valid!"))]
    pub email_address: String,
    pub mobile_number: Option<String>,
    pub enabled: bool,
    pub email_confirmed: bool,
    pub role_id: i16,
    pub created_at: DateTime<Utc>,
}

impl<'c> FromRow<'c, PgRow> for User {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(User {
            user_id: row.get(0),
            first_name: row.get(1),
            middle_name: row.get(2),
            surname: row.get(3),
            email_address: row.get(4),
            mobile_number: row.get(5),
            enabled: row.get(6),
            email_confirmed: row.get(7),
            role_id: row.get(8),
            created_at: row.get(9),
        })
    }
}
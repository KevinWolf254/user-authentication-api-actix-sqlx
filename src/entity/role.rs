use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, postgres::PgRow, Row};
use validator::Validate;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub role_id: i16,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateRole {
    #[validate(length(min = 3, message = "Role name is required!"))]
    pub name: String,
}

impl<'c> FromRow<'c, PgRow> for Role {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Role {
            role_id: row.get(0),
            name: row.get(1),
            created_at: row.get(2),
        })
    }
}
pub mod db_context;
pub mod permission_dao;
pub mod role_dao;

pub type Database<'c> = db_context::Database<'c>;
pub type Table<'c, T> = db_context::Table<'c, T>;

pub struct CountResult {
    pub count: Option<i64>
}
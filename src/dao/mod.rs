pub mod db_context;
pub mod permission_dao;
pub mod role_dao;
pub mod role_permission_dao;
pub mod user_dao;
pub mod user_credential_dao;

pub type Database<'c> = db_context::Database<'c>;
pub type Table<'c, T> = db_context::Table<'c, T>;
pub type JoinTable<'c, T1, T2> = db_context::JoinTable<'c, T1, T2>;

pub struct CountResult {
    pub count: Option<i64>
}
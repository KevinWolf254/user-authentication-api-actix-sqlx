pub mod dao_context;
pub mod permission_dao;

// pub type Database<'c> = db_context::Database<'c>;
pub type Table<'c, T> = dao_context::Table<'c, T>;

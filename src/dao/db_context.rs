use std::marker::PhantomData;
use std::sync::Arc;

use sqlx::{ FromRow, PgPool };
use sqlx::postgres::{ PgRow, PgPoolOptions };

use crate::entity::permission::Permission;

pub struct Table<'c, T> where T: FromRow<'c, PgRow> {
    pub pool: Arc<PgPool>,
    _from_row: fn(&'c PgRow) -> Result<T, sqlx::Error>,
    _marker: PhantomData<&'c T>,
}

impl<'c, T> Table<'c, T> where T: FromRow<'c, PgRow> {
    fn new(pool: Arc<PgPool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row,
            _marker: PhantomData,
        }
    }
}

pub struct Database<'c> {
    pub permissions: Arc<Table<'c, Permission>>,
}

impl<'a> Database<'a> {
    pub async fn new(database_url: &String, max_connections: u32) -> Database<'a> {
        let connection = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(&database_url).await
            .expect("Unable to connect to the database!");
        let pool = Arc::new(connection);

        Database {
            permissions: Arc::from(Table::new(pool.clone())),
        }
    }
}

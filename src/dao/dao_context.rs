use std::marker::PhantomData;
use std::sync::Arc;

use sqlx::{FromRow, PgPool};
use sqlx::postgres::PgRow;

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
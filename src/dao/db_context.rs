use std::marker::PhantomData;
use std::sync::Arc;

use sqlx::{ FromRow, PgPool, Pool };
use sqlx::postgres::{ PgRow, PgPoolOptions };

use crate::entity::permission::Permission;
use crate::entity::role::Role;
use crate::entity::user::User;
use crate::entity::user_credential::UserCredential;

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

pub struct JoinTable<'c, T1, T2>
where
    T1: FromRow<'c, PgRow>,
    T2: FromRow<'c, PgRow>,
{
    pub pool: Arc<PgPool>,
    _from_row: (
        fn(&'c PgRow) -> Result<T1, sqlx::Error>,
        fn(&'c PgRow) -> Result<T2, sqlx::Error>,
    ),
    _marker_t1: PhantomData<&'c T1>,
    _marker_t2: PhantomData<&'c T2>,
}

impl<'c, T1, T2> JoinTable<'c, T1, T2>
where
    T1: FromRow<'c, PgRow>,
    T2: FromRow<'c, PgRow>,
{
    fn new(pool: Arc<PgPool>) -> Self {
        JoinTable {
            pool,
            _from_row: (T1::from_row, T2::from_row),
            _marker_t1: PhantomData,
            _marker_t2: PhantomData,
        }
    }
}

pub struct Database<'c> {
    pub permissions: Arc<Table<'c, Permission>>,
    pub roles: Arc<Table<'c, Role>>,    
    pub role_permissions: Arc<JoinTable<'c, Role, Permission>>,
    pub users: Arc<Table<'c, User>>,  
    pub user_credentials: Arc<Table<'c, UserCredential>>,  
}

impl<'a> Database<'a> {

    pub async fn new(database_url: &String, max_connections: u32) -> Database<'a> {
        let connection = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(&database_url).await
            .expect("Unable to connect to the database!");

        let pool: Arc<sqlx::Pool<sqlx::Postgres>> = Arc::new(connection);

        Database {
            permissions: Arc::from(Table::new(pool.clone())),
            roles: Arc::from(Table::new(pool.clone())),
            role_permissions: Arc::from(JoinTable::new(pool.clone())),
            users: Arc::from(Table::new(pool.clone())),
            user_credentials: Arc::from(Table::new(pool.clone())),
        }
    }

    pub async fn test(pool: Pool<sqlx::Postgres>) -> Database<'a> {
        Database {
            permissions: Arc::from(Table::new(Arc::new(pool.clone()))),
            roles: Arc::from(Table::new(Arc::new(pool.clone()))),
            role_permissions: Arc::from(JoinTable::new(Arc::new(pool.clone()))),
            users: Arc::from(Table::new(Arc::new(pool.clone()))),
            user_credentials: Arc::from(Table::new(Arc::new(pool.clone()))),
        }
    }
}

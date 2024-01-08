use std::env;

use bulk_sms_api::dao::Database;
use dotenv::dotenv;

pub async fn init_test_db() -> Database<'static> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL has not been set!");

    Database::new(&database_url, 5).await
}

#[cfg(test)]
mod dao_tests;
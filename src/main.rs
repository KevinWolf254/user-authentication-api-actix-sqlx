use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{ web, App, HttpServer };
use dotenv::dotenv;
use handler::user_handler::{ create_user, delete_user, get_user, update_user };
use sqlx::{ postgres::PgPoolOptions, Pool, Postgres };
use std::env;

mod handler;
mod model;
mod dao;

pub struct AppState {
    // pub connections: Mutex<u32>,
    // pub context: Arc<Database<'a>>,
    pub db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL has not been set!");
    let max_connections = env::var("MAX_CONNECTIONS")
        .unwrap_or_else(|_| "5".to_string()) // Default to 5 if not present
        .parse()
        .expect("Failed to parse MAX_CONNECTIONS as u32");

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&database_url).await
        .expect("Unable to connect to the database!");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(
                web
                    ::scope("api/v1/")
                    .configure(handler::init_permission_handler)
                    .service(get_user)
                    .service(create_user)
                    .service(update_user)
                    .service(delete_user)
            )
    }).bind(format!("127.0.0.1:{}", server_port))?;

    // Run the server
    server.run().await?;

    Ok(())
}

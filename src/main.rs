// use actix_web::middleware::Logger;
use actix_web::{ web, App, HttpServer };
use dao::Database;
use dotenv::dotenv;
use handler::user_handler::{ create_user, delete_user, get_user, update_user };
use slog::{Logger, Drain, o, info};
use std::env;
use std::sync::{Arc, Mutex};

mod handler;
mod model;
mod dao;
mod error;

pub struct AppState<'a> {
    pub connections: Mutex<u32>,
    pub context: Arc<Database<'a>>,
    pub log: Logger
}

fn configure_log() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let log = configure_log();
    info!(log, "Loading environment variables...");

    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL has not been set!");
    let max_connections = env::var("MAX_CONNECTIONS")
        .unwrap_or_else(|_| "5".to_string()) // Default to 5 if not present
        .parse()
        .expect("Failed to parse MAX_CONNECTIONS as u32");

    info!(log, "Starting server at http://localhost:{}", server_port);

    let db_context = Database::new(&database_url, max_connections).await;

    let app_state = web::Data::new(AppState {
        connections: Mutex::new(0),
        context: Arc::new(db_context),
        log: log.clone()
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(
                web
                    ::scope("api/v1")
                    .configure(handler::init_permission_handler)
                    .service(get_user)
                    .service(create_user)
                    .service(update_user)
                    .service(delete_user)
            )
    }).bind(format!("127.0.0.1:{}", server_port))
    .and_then(|result| {
        info!(log, "Successfully started the server");
        Ok(result)
    })?;

    // Run the server
    server.run().await
    .and_then(|_| {
        info!(log, "Stopped the server");
        Ok(())
    })?;

    Ok(())
}

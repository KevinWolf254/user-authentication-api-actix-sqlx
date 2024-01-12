use actix_web::{ web, App, HttpServer };
use bulk_sms_api::{AppState, handler};
use bulk_sms_api::dao::Database;
use dotenvy::dotenv;
use slog::{Logger, Drain, o, info, warn};
use std::env;
use std::fs::OpenOptions;
use std::net::Ipv4Addr;
use std::sync::{Arc, Mutex};

fn configure_log(log_path: String) -> Logger {
    let file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(log_path)
      .unwrap();
    let decorator = slog_term::PlainDecorator::new(file);
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    const DEFAULT_SERVER_PORT: u16 = 8080;
    const DEFAULT_MAX_CONNECTIONS: u32 = 5;
    const DEFAULT_LOG_PATH: &str = "log/sms_gateway.log";

    let log_path = env::var("LOG_PATH").unwrap_or_else(|_| DEFAULT_LOG_PATH.to_string());

    let log = configure_log(log_path);

    let server_port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| {
            warn!(log, "SERVER_PORT env variable was not provided. Will default to {}", DEFAULT_SERVER_PORT);
            DEFAULT_SERVER_PORT.to_string()
        })
        .parse()
        .unwrap_or_else(|_| {
            warn!(log, "SERVER_PORT was not of type u16. Will default to {}", DEFAULT_SERVER_PORT);
            DEFAULT_SERVER_PORT
        });

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL has not been set!");

    let max_connections = env::var("MAX_CONNECTIONS")
        .unwrap_or_else(|_| {
            warn!(log, "MAX_CONNECTIONS env variable was not provided. Will default to {}", DEFAULT_MAX_CONNECTIONS);
            DEFAULT_MAX_CONNECTIONS.to_string()
        }) // Default to 5 if not present
        .parse()
        .unwrap_or_else(|_| {
            warn!(log, "MAX_CONNECTIONS was not of type u32. Will default to {}", DEFAULT_MAX_CONNECTIONS);
            DEFAULT_MAX_CONNECTIONS
        });

    let localhost = Ipv4Addr::new(127, 0, 0, 1);
    
    info!(log, "Starting server at http://{:?}:{}", localhost, server_port);

    let db_context = Database::new(&database_url, max_connections).await;

    let app_state = web::Data::new(AppState {
        connections: Mutex::new(0),
        context: Arc::new(db_context),
        log: Arc::new(log.clone())
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(
                web
                    ::scope("api/v1")
                    .configure(handler::init_permission_handler)
            )
    }).bind((localhost, server_port))
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

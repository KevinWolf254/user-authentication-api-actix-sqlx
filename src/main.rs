use actix_web::{ web, App, HttpServer };
use bulk_sms_api::{handler, AppState, JwtConfig};
use bulk_sms_api::dao::Database;
use dotenvy::dotenv;
use log::{info, warn};
use std::env;
use std::net::Ipv4Addr;
use std::sync::Arc;
use argon2::Config;

extern crate argon2;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    const DEFAULT_SERVER_PORT: u16 = 8080;
    const DEFAULT_MAX_CONNECTIONS: u32 = 5;

    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let server_port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| {
            warn!("SERVER_PORT env variable was not provided. Will default to {}", DEFAULT_SERVER_PORT);
            DEFAULT_SERVER_PORT.to_string()
        })
        .parse()
        .unwrap_or_else(|_| {
            warn!("SERVER_PORT was not of type u16. Will default to {}", DEFAULT_SERVER_PORT);
            DEFAULT_SERVER_PORT
        });

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL has not been set!");

    let max_connections = env::var("MAX_CONNECTIONS")
        .unwrap_or_else(|_| {
            warn!("MAX_CONNECTIONS env variable was not provided. Will default to {}", DEFAULT_MAX_CONNECTIONS);
            DEFAULT_MAX_CONNECTIONS.to_string()
        }) // Default to 5 if not present
        .parse()
        .unwrap_or_else(|_| {
            warn!("MAX_CONNECTIONS was not of type u32. Will default to {}", DEFAULT_MAX_CONNECTIONS);
            DEFAULT_MAX_CONNECTIONS
        });
        
    let config = Config::default();
    
    let localhost = Ipv4Addr::new(127, 0, 0, 1);
    
    info!("Starting server at http://{:?}:{}", localhost, server_port);

    let db_context = Database::new(&database_url, max_connections).await;

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET was not provided.");
    let expires_in = env::var("JWT_EXPIRES_IN").expect("JWT_EXPIRES_IN was not provided.").parse::<i64>().expect("JWT_EXPIRES_IN should be an i32.");
    
    let jwt_config = JwtConfig {secret, expires_in};
    
    let app_state = web::Data::new(AppState {
        context: Arc::new(db_context),
        argon_config: Arc::new(config),
        jwt_config: Arc::new(jwt_config),
    });

    let server = HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(app_state.clone())
            .service(
                web
                    ::scope("")
                    .configure(handler::init_auth_handler)
            )
            .service(
                web
                    ::scope("api/v1")
                    .configure(handler::init_permission_handler)
                    .configure(handler::init_role_handler)
                    .configure(handler::init_user_handler)
            )
    }).bind((localhost, server_port))
    .and_then(|result| {
        info!("Successfully started the server");
        Ok(result)
    })?;

    // Run the server
    server.run().await
    .and_then(|_| {
        info!("Stopped the server");
        Ok(())
    })?;

    Ok(())
}

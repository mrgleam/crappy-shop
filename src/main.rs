mod application;
mod config;
mod database;
mod domain;
mod environment;
mod infrastructure;
mod middleware;

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use config::Config;
use env_logger::Env;
use infrastructure::AppState;
use middleware::JwtMiddleware;
use migration::{Migrator, MigratorTrait};
use std::{io::Error, sync::Arc};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("up-to-me!")
}

fn main() -> Result<(), Error> {
    actix_web::rt::System::new().block_on(async { init().await })
}

async fn init() -> Result<(), Error> {
    let config = Config::load();
    let conn = database::new(&config.database).await;
    Migrator::up(&conn, None).await.expect("Failed to migrate");

    let state = AppState {
        conn: Arc::new(conn),
        authentication_config: Arc::new(config.authentication),
    };

    let addr = (config.http_server.host.as_str(), config.http_server.port);

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(state.clone()))
            .service(health)
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/public/resources")
                            .configure(infrastructure::user::public_configure)
                            .configure(infrastructure::product::configure),
                    )
                    .service(
                        web::scope("/protected/resources")
                            .wrap(JwtMiddleware::new(
                                state.authentication_config.secret.as_str(),
                            ))
                            .configure(infrastructure::user::protected_configure),
                    ),
            )
    })
    .bind(addr)?
    .run()
    .await
}

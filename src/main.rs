mod application;
mod config;
mod database;
mod domain;
mod environment;
mod infrastructure;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use config::Config;
use infrastructure::AppState;
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

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(health)
            .service(
                web::scope("/api")
                    .configure(infrastructure::user::configure)
                    .configure(infrastructure::product::configure),
            )
    })
    .bind(addr)?
    .run()
    .await
}

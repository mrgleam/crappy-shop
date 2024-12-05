mod infrastructure;
mod database;

use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};
use infrastructure::AppState;
use std::io::Error;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("up-to-me!")
}

fn main() -> Result<(), Error> {
    actix_web::rt::System::new().block_on(async { init().await })
}

async fn init() -> Result<(), Error> {
    let conn = database::new().await;
    let state = AppState { conn };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(health)
            .service(
                web::scope("/api")
                    .configure(infrastructure::user::configure)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
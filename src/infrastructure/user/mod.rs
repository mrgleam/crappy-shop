mod error;
mod handler;
mod input;
pub mod repository;

use actix_web::web;
use actix_web::web::get;
use actix_web::web::patch;
use actix_web::web::post;
use actix_web::web::ServiceConfig;

pub fn public_configure(config: &mut ServiceConfig) {
    config.service(
        web::scope("/users")
            .route("", get().to(handler::index))
            .route("{id}", get().to(handler::get_by_id))
            .route("", patch().to(handler::update))
            .route("/signup", post().to(handler::create))
            .route("/signin", post().to(handler::signin)),
    );
}

pub fn protected_configure(config: &mut ServiceConfig) {
    config.service(
        web::scope("/users")
            .route("", get().to(handler::index))
            .route("{id}", get().to(handler::get_by_id))
            .route("", patch().to(handler::update)),
    );
}

mod error;
mod handler;
pub mod repository;

use actix_web::web;
use actix_web::web::post;
use actix_web::web::ServiceConfig;

pub fn protected_configure(config: &mut ServiceConfig) {
    config.service(web::scope("/items").route("", post().to(handler::add_item)));
}

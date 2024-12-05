mod handler;
mod repository;

use actix_web::web;
use actix_web::web::get;
use actix_web::web::ServiceConfig;

pub fn configure(config: &mut ServiceConfig) {
    config.service(web::scope("/users")
      .route("",get().to(handler::index))
      .route("{id}",get().to(handler::get_by_id))
    );
}
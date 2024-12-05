use actix_web::{
    body::BoxBody, http::header::ContentType, web, HttpRequest, HttpResponse, Responder
};
use serde::Serialize;
use sea_orm::EntityTrait;

use crate::infrastructure::AppState;

use super::repository::Entity as UserEntity;

#[derive(Serialize)]
struct User {
    id: String,
    email: String,
}

impl Responder for User {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

pub async fn index(
    db: web::Data<AppState>,
) -> impl Responder {
    let users = UserEntity::find().all(&db.conn).await;
    match users {
        Ok(users) => {
            let users: Vec<User> = users.iter()
                .map(|user| User { 
                    id: user.id.to_string(), 
                    email: user.email.clone() 
                })
                .collect();
            HttpResponse::Ok().json(users)
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_by_id(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    User { id: id, email: "user".into() }
}

use crate::domain::user::view::UserView;
use crate::infrastructure::AppState;
use actix_web::{web, HttpResponse, Responder};
use entity::user;
use sea_orm::EntityTrait;

pub async fn index(db: web::Data<AppState>) -> impl Responder {
    let users = user::Entity::find().all(&db.conn).await;
    match users {
        Ok(users) => {
            let users: Vec<UserView> = users
                .iter()
                .map(|user| UserView {
                    id: Some(user.id.to_string()),
                    email: user.email.clone(),
                })
                .collect();
            HttpResponse::Ok().json(users)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_by_id(path: web::Path<i32>, db: web::Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();
    let user = user::Entity::find_by_id(user_id).one(&db.conn).await;

    match user {
        Ok(user) => match user {
            Some(user) => HttpResponse::Ok().json(UserView {
                id: Some(user.id.to_string()),
                email: user.email.clone(),
            }),
            None => HttpResponse::NotFound().body("User not found"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

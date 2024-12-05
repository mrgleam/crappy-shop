use crate::infrastructure::AppState;
use crate::{domain::user::view::UserView, infrastructure::response};
use actix_web::{web, Responder};
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
            response::Default::new(users).json()
        }
        Err(e) => response::Error::new(e.to_string()).json(),
    }
}

pub async fn get_by_id(path: web::Path<i32>, db: web::Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();
    let user = user::Entity::find_by_id(user_id).one(&db.conn).await;

    match user {
        Ok(user) => match user {
            Some(user) => response::Default::new(UserView {
                id: Some(user.id.to_string()),
                email: user.email,
            })
            .json(),
            None => response::Error::new("User not found".into()).json(),
        },
        Err(_) => response::Error::new("Internal server error".into()).json(),
    }
}

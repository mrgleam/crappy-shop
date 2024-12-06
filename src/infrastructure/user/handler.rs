use crate::application::user::appregate::Appregate;
use crate::application::user::command::UserCommand;
use crate::application::user::service::UserService;
use crate::domain::user::aggregate::User;
use crate::domain::user::view::UserView;
use crate::infrastructure::response;
use crate::infrastructure::AppState;
use actix_web::{web, Responder};
use entity::user;
use sea_orm::EntityTrait;

use super::input::CreateUser;
use super::input::UpdateUser;
use super::repository::UserRepository;

pub async fn index(db: web::Data<AppState>) -> impl Responder {
    let users = user::Entity::find().all(db.conn.as_ref()).await;
    match users {
        Ok(users) => {
            let users: Vec<UserView> = users
                .iter()
                .map(|user| UserView {
                    id: Some(user.id.to_string()),
                    email: user.email.clone(),
                    ..Default::default()
                })
                .collect();
            response::Default::new(users).json()
        }
        Err(e) => response::Error::new(e.to_string()).json(),
    }
}

pub async fn get_by_id(path: web::Path<i32>, db: web::Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();
    let user = user::Entity::find_by_id(user_id)
        .one(db.conn.as_ref())
        .await;

    match user {
        Ok(user) => match user {
            Some(user) => response::Default::new(UserView {
                id: Some(user.id.to_string()),
                email: user.email,
                ..Default::default()
            })
            .json(),
            None => response::Error::new("User not found".into()).json(),
        },
        Err(_) => response::Error::new("Internal server error".into()).json(),
    }
}

pub async fn create(body: web::Json<CreateUser>, db: web::Data<AppState>) -> impl Responder {
    let user = body.into_inner();
    let repository = UserRepository::new(db.conn.clone());
    let service = UserService::new(repository);
    let user_event = User::new()
        .handle(UserCommand::create(user.email, user.password), &service)
        .await;
    match user_event {
        Ok(_) => response::Created::into(),
        Err(_) => response::Error::new("Internal server error".into()).json(),
    }
}

pub async fn update(body: web::Json<UpdateUser>, db: web::Data<AppState>) -> impl Responder {
    let user = body.into_inner();
    let repository = UserRepository::new(db.conn.clone());
    let service = UserService::new(repository);
    let user_event = User::new()
        .handle(
            UserCommand::update(user.id, user.email, user.password),
            &service,
        )
        .await;
    match user_event {
        Ok(event) => {
            let mut user = User::new();
            user.apply(event);
            response::Default::new(UserView::from(user)).json()
        }
        Err(_) => response::Error::new("Internal server error".into()).json(),
    }
}

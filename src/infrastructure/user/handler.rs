use crate::application::user::appregate::Appregate;
use crate::application::user::command::UserCommand;
use crate::application::user::service::UserService;
use crate::domain::user::aggregate::User;
use crate::domain::user::view::UserView;
use crate::infrastructure::error::APIError;
use crate::infrastructure::response;
use crate::infrastructure::AppState;
use actix_web::ResponseError;
use actix_web::{web, Responder};

use super::input::CreateUser;
use super::input::SignInUser;
use super::input::UpdateUser;
use super::repository::UserRepository;
pub async fn index(db: web::Data<AppState>) -> impl Responder {
    let users: Vec<UserView> = UserRepository::new(db.conn.clone())
        .find_all()
        .await
        .iter()
        .map(|user| UserView {
            id: Some(user.id.to_string()),
            email: user.email.clone(),
            ..Default::default()
        })
        .collect();
    response::Default::new(users).json()
}

pub async fn get_by_id(path: web::Path<i32>, db: web::Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();

    let repository = UserRepository::new(db.conn.clone());
    let service: UserService = UserService::new(repository);
    let user = service.get_by_id(user_id).await;

    match user {
        Ok(user) => response::Default::new(user).json(),
        Err(e) => APIError::from(e).error_response(),
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
        Err(e) => APIError::from(e).error_response(),
    }
}

pub async fn update(body: web::Json<UpdateUser>, db: web::Data<AppState>) -> impl Responder {
    let update_user = body.into_inner();
    let repository = UserRepository::new(db.conn.clone());
    let service = UserService::new(repository);
    let mut user = User::new();
    let user_event = user
        .handle(
            UserCommand::update(update_user.id, update_user.email, update_user.password),
            &service,
        )
        .await
        .map(|event| {
            user.apply(event);
        });
    match user_event {
        Ok(_) => response::Default::new(UserView::from(user)).json(),
        Err(e) => APIError::from(e).error_response(),
    }
}

pub async fn signin(body: web::Json<SignInUser>, db: web::Data<AppState>) -> impl Responder {
    let signin_user = body.into_inner();
    let repository = UserRepository::new(db.conn.clone());
    let service = UserService::new(repository);
    let mut user = User::new();
    let user_event = user
        .handle(
            UserCommand::signin(signin_user.email, signin_user.password),
            &service,
        )
        .await
        .map(|event| {
            user.apply(event);
        });
    match user_event {
        Ok(_) => response::Default::new(UserView::from(user)).json(),
        Err(e) => APIError::from(e).error_response(),
    }
}

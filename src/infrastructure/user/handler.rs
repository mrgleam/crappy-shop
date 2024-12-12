use crate::application::user::aggregate::Aggregate;
use crate::application::user::command::UserCommand;
use crate::application::user::service::UserService;
use crate::domain::user::aggregate::User;
use crate::domain::user::view::UserView;
use crate::infrastructure::error::APIError;
use crate::infrastructure::response;
use crate::infrastructure::AppState;
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

pub async fn get_by_id(
    path: web::Path<i32>,
    db: web::Data<AppState>,
) -> Result<impl Responder, APIError> {
    let user_id = path.into_inner();

    let repository: UserRepository = UserRepository::new(db.conn.clone());
    let service: UserService = UserService::new(repository, db.authentication_config.clone());

    Ok(response::Default::new(service.get_by_id(user_id).await?).json())
}

pub async fn create(
    body: web::Json<CreateUser>,
    db: web::Data<AppState>,
) -> Result<impl Responder, APIError> {
    let user = body.into_inner();
    let repository = UserRepository::new(db.conn.clone());
    let service = UserService::new(repository, db.authentication_config.clone());
    User::new()
        .handle(UserCommand::create(user.email, user.password), &service)
        .await?;

    Ok(response::Created::into())
}

pub async fn update(
    body: web::Json<UpdateUser>,
    db: web::Data<AppState>,
) -> Result<impl Responder, APIError> {
    let update_user = body.into_inner();
    let repository = UserRepository::new(db.conn.clone());
    let service = UserService::new(repository, db.authentication_config.clone());
    let mut user = User::new();
    user.handle(
        UserCommand::update(update_user.id, update_user.email, update_user.password),
        &service,
    )
    .await
    .map(|event| {
        user.apply(event);
    })?;

    Ok(response::Default::new(UserView::from(user)).json())
}

pub async fn signin(
    body: web::Json<SignInUser>,
    db: web::Data<AppState>,
) -> Result<impl Responder, APIError> {
    let signin_user = body.into_inner();
    let repository = UserRepository::new(db.conn.clone());
    let service = UserService::new(repository, db.authentication_config.clone());
    let mut user = User::new();
    user.handle(
        UserCommand::signin(signin_user.email, signin_user.password),
        &service,
    )
    .await
    .map(|event| {
        user.apply(event);
    })?;

    Ok(response::Default::new(UserView::from(user)).json())
}

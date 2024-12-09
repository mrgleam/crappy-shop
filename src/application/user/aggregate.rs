use async_trait::async_trait;
use chrono::Utc;
use validator::Validate;

use crate::domain::user::{aggregate::User, error::UserError, event::UserEvent};

use super::{command::UserCommand, service::UserService};

#[async_trait]
pub trait Aggregate {
    type Command;
    type Event;
    type Service;
    async fn handle(
        &self,
        command: Self::Command,
        service: &Self::Service,
    ) -> Result<Self::Event, UserError>;
    fn apply(&mut self, event: Self::Event);
}

#[async_trait]
impl Aggregate for User {
    type Command = UserCommand;
    type Event = UserEvent;
    type Service = UserService;

    async fn handle(
        &self,
        command: Self::Command,
        service: &Self::Service,
    ) -> Result<Self::Event, UserError> {
        match command {
            UserCommand::Create { email, password } => {
                let now = Utc::now();
                let mut user = User {
                    email,
                    password,
                    created_at: now,
                    updated_at: now,
                    token: None,
                    ..(*self)
                };

                user.validate()
                    .map_err(|e: validator::ValidationErrors| UserError::from(e))?;

                service.encrypted_password(&mut user)?;

                let created = service.create(&user).await;

                match created {
                    Ok(id) => Ok(UserEvent::Created {
                        id,
                        email: user.email,
                        date: now,
                    }),
                    Err(e) => Err(e),
                }
            }
            UserCommand::Update {
                id,
                email,
                password,
            } => {
                let now = Utc::now();
                let mut user = User {
                    id: Some(id),
                    email,
                    password,
                    created_at: now,
                    updated_at: now,
                    token: None,
                };

                user.validate()
                    .map_err(|e: validator::ValidationErrors| UserError::from(e))?;

                service.encrypted_password(&mut user)?;

                let updated = service.update(&user).await;
                match updated {
                    Ok(_) => Ok(UserEvent::Updated {
                        id,
                        email: user.email,
                        date: now,
                    }),
                    Err(e) => Err(e),
                }
            }
            UserCommand::SignIn { email, password } => {
                let user = User {
                    email: email.clone(),
                    password,
                    token: None,
                    ..(*self)
                };
                user.validate()
                    .map_err(|e: validator::ValidationErrors| UserError::from(e))?;
                let logged_in = service.signin(&user.email, &user.password).await?;
                let token = service.create_token(logged_in)?;
                Ok(UserEvent::LoggedIn {
                    email: user.email,
                    token,
                })
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            UserEvent::Created { id, email, date } => {
                self.id = Some(id);
                self.email = email;
                self.created_at = date;
            }
            UserEvent::Updated { id, email, date } => {
                self.id = Some(id);
                self.email = email;
                self.updated_at = date;
            }
            UserEvent::LoggedIn { email, token } => {
                self.email = email;
                self.token = Some(token);
            }
        }
    }
}

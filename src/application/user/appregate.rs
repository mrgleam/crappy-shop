use async_trait::async_trait;
use chrono::Utc;

use crate::domain::user::{aggregate::User, event::UserEvent};

use super::{command::UserCommand, service::UserService};

#[async_trait]
pub trait Appregate {
    type Command;
    type Event;
    type Service;
    async fn handle(
        &self,
        command: Self::Command,
        service: &Self::Service,
    ) -> Result<Self::Event, String>;
    fn apply(&mut self, event: Self::Event);
}

#[async_trait]
impl Appregate for User {
    type Command = UserCommand;
    type Event = UserEvent;
    type Service = UserService;

    async fn handle(
        &self,
        command: Self::Command,
        service: &Self::Service,
    ) -> Result<Self::Event, String> {
        match command {
            UserCommand::Create { email, password } => {
                let now = Utc::now();
                let mut user = User {
                    email,
                    password,
                    created_at: now,
                    updated_at: now,
                    ..(*self)
                };

                service
                    .encrypted_password(&mut user)
                    .map_err(|e| e.to_string())?;

                let created = service.create(&user).await;

                match created {
                    Ok(id) => Ok(UserEvent::Created {
                        id,
                        email: user.email,
                        date: now,
                    }),
                    Err(e) => Err(e.to_string()),
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
                    ..(*self)
                };
                service
                    .encrypted_password(&mut user)
                    .map_err(|e| e.to_string())?;

                let updated = service.update(&user).await;
                match updated {
                    Ok(_) => Ok(UserEvent::Updated {
                        id,
                        email: user.email,
                        date: now,
                    }),
                    Err(e) => Err(e.to_string()),
                }
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
        }
    }
}

use async_trait::async_trait;

use crate::domain::cart::{aggregate::Cart, error::CartError, event::CartEvent};

use super::{command::CartCommand, service::CartService};

#[async_trait]
pub trait Aggregate {
    type Command;
    type Event;
    type Service;
    async fn handle(
        &mut self,
        command: Self::Command,
        service: &Self::Service,
    ) -> Result<Self::Event, CartError>;
    fn apply(&mut self, event: Self::Event);
}

#[async_trait]
impl Aggregate for Cart {
    type Command = CartCommand;
    type Event = CartEvent;
    type Service = CartService;

    async fn handle(
        &mut self,
        command: Self::Command,
        service: &Self::Service,
    ) -> Result<Self::Event, CartError> {
        match command {
            CartCommand::AddItem(item) => {
                service.add_item(item).await?;
                let event = CartEvent::ItemAdded;
                Ok(event)
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            CartEvent::ItemAdded => {
                self;
            }
        }
    }
}

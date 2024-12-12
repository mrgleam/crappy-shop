use crate::domain::cart::aggregate::CartItem;

pub enum CartCommand {
    AddItem(CartItem),
}

impl CartCommand {
    pub fn add_item(item: CartItem) -> Self {
        CartCommand::AddItem(item)
    }
}

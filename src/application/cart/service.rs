use crate::{
    domain::cart::{
        aggregate::{Cart, CartItem},
        error::CartError,
    },
    infrastructure::cart::repository::CartRepository,
};

pub struct CartService {
    repository: CartRepository,
}

impl CartService {
    pub fn new(repository: CartRepository) -> Self {
        Self { repository }
    }

    pub async fn get_cart_by_user_id(&self, user_id: i32) -> Result<Cart, CartError> {
        let result = self.repository.get_cart_by_user_id(user_id).await;
        match result {
            Ok(Some((model, items))) => {
                let mut cart = Cart::new();
                cart.id = Some(model.id);
                cart.user_id = model.user_id;
                cart.created_at = model.created_at.to_utc();
                cart.updated_at = model.updated_at.to_utc();
                cart.items = items
                    .into_iter()
                    .map(|model| {
                        let mut item = CartItem::new();
                        item.id = Some(model.id);
                        item.cart_id = model.cart_id;
                        item.product_id = model.product_id;
                        item.quantity = model.quantity as u8;
                        item.created_at = model.created_at.to_utc();
                        item.updated_at = model.updated_at.to_utc();
                        item
                    })
                    .collect();
                Ok(cart)
            }
            Ok(None) => Err(CartError::NotFound),
            Err(e) => Err(CartError::from(e)),
        }
    }

    pub async fn create_cart(&self, user_id: i32) -> Result<i32, CartError> {
        let created = self.repository.create_cart(user_id).await;
        match created {
            Ok(result) => Ok(result.last_insert_id),
            Err(e) => Err(CartError::from(e)),
        }
    }

    pub async fn add_item(&self, item: CartItem) -> Result<i32, CartError> {
        let created = self.repository.add_item(&item).await;
        match created {
            Ok(result) => Ok(result.last_insert_id),
            Err(e) => Err(CartError::from(e)),
        }
    }
}

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Default)]
pub struct Cart {
    pub id: Option<i32>,
    pub user_id: i32,
    pub items: Vec<CartItem>,
    pub price: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    max_items_limit: u8,
    last_updated_at: DateTime<Utc>,
}

impl Cart {
    pub fn new() -> Self {
        Self {
            max_items_limit: 10,
            last_updated_at: Utc::now(),
            ..Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn get_total_items(&self) -> u8 {
        self.items.len().try_into().unwrap_or(self.max_items_limit)
    }

    pub fn add_item(&mut self, item: CartItem) -> Result<(), &str> {
        if self.is_full() {
            Err("Cart is full")
        } else {
            self.items.push(item);
            self.update_price();
            self.update_last_updated_at();
            Ok(())
        }
    }

    pub fn update_last_updated_at(&mut self) {
        self.last_updated_at = Utc::now();
    }

    fn is_full(&self) -> bool {
        self.get_total_items() >= self.max_items_limit
    }

    fn update_price(&mut self) {
        self.price = self
            .items
            .iter()
            .map(|item| item.price * Decimal::from(item.quantity))
            .sum();
    }
}

#[derive(Serialize, Deserialize, Validate, Default)]
pub struct CartItem {
    pub id: Option<i32>,
    pub product_id: i32,
    pub cart_id: i32,
    pub name: String,
    pub price: Decimal,
    pub quantity: u8,
    max_quantity: u8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CartItem {
    pub fn new() -> Self {
        Self {
            max_quantity: 10,
            ..Default::default()
        }
    }

    pub fn is_full(&self) -> bool {
        self.quantity >= self.max_quantity
    }

    pub fn update_quantity(&mut self, quantity: u8) -> Result<(), &str> {
        if self.is_full() {
            Err("Cart can't have more than max quantity defined")
        } else {
            self.quantity = quantity;
            Ok(())
        }
    }
}

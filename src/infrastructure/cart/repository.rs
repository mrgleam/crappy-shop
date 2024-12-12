use std::sync::Arc;

use chrono::Utc;
use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, InsertResult, QueryFilter,
    UpdateResult,
};

use crate::domain::cart::aggregate::CartItem;

use entity::{cart, cart_item};

pub struct CartRepository {
    pub db: Arc<DatabaseConnection>,
}

impl CartRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn get_cart_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<Option<(cart::Model, Vec<cart_item::Model>)>, DbErr> {
        let result = cart::Entity::find()
            .filter(cart::Column::UserId.eq(user_id))
            .find_with_related(cart_item::Entity)
            .all(self.db.as_ref())
            .await?;
        Ok(result.into_iter().next())
    }

    pub async fn create_cart(
        &self,
        user_id: i32,
    ) -> Result<InsertResult<cart::ActiveModel>, DbErr> {
        let model = cart::ActiveModel {
            id: ActiveValue::NotSet,
            user_id: ActiveValue::Set(user_id),
            created_at: ActiveValue::Set(Utc::now().into()),
            updated_at: ActiveValue::Set(Utc::now().into()),
        };
        cart::Entity::insert(model).exec(self.db.as_ref()).await
    }

    pub async fn add_item(
        &self,
        item: &CartItem,
    ) -> Result<InsertResult<cart_item::ActiveModel>, DbErr> {
        let model = cart_item::ActiveModel {
            id: ActiveValue::NotSet,
            cart_id: ActiveValue::Set(item.cart_id),
            product_id: ActiveValue::Set(item.product_id),
            quantity: ActiveValue::Set(item.quantity as i16),
            created_at: ActiveValue::Set(item.created_at.into()),
            updated_at: ActiveValue::Set(item.updated_at.into()),
        };
        cart_item::Entity::insert(model)
            .exec(self.db.as_ref())
            .await
    }
}

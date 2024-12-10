use std::sync::Arc;

use entity::product;

use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
pub struct ProductRepository {
    pub db: Arc<DatabaseConnection>,
}

impl ProductRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Vec<product::Model> {
        let result = product::Entity::find()
            .filter(product::Column::DeletedAt.is_null())
            .all(self.db.as_ref())
            .await;
        result.unwrap_or_default()
    }

    pub async fn find_by_id(&self, id: i32) -> Result<product::Model, DbErr> {
        let result = product::Entity::find_by_id(id)
            .filter(product::Column::DeletedAt.is_null())
            .one(self.db.as_ref())
            .await;

        match result {
            Ok(Some(m)) => Ok(m),
            _ => Err(DbErr::RecordNotFound("Product not found".into())),
        }
    }
}

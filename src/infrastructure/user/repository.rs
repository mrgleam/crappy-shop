use std::sync::Arc;

use entity::user;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use sea_orm::ActiveModelTrait;

use sea_orm::TryIntoModel;

use crate::domain::user::aggregate::User;

pub struct UserRepository {
    pub db: Arc<DatabaseConnection>,
}

impl UserRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Vec<user::Model> {
        let result = user::Entity::find().all(self.db.as_ref()).await;

        result.unwrap_or_default()
    }

    pub async fn find_by_id(&self, id: i32) -> Result<user::Model, DbErr> {
        let result = user::Entity::find_by_id(id).one(self.db.as_ref()).await;

        match result {
            Ok(Some(m)) => Ok(m),
            _ => Err(DbErr::RecordNotFound("User not found".into())),
        }
    }

    pub async fn save(&self, user: User) -> Result<user::Model, DbErr> {
        let active_model: user::ActiveModel = user.into();
        let saved = active_model.save(self.db.as_ref()).await?;
        saved.try_into_model()
    }

    pub async fn find_by_email(&self, email: &str) -> Result<user::Model, DbErr> {
        let result = user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(self.db.as_ref())
            .await;
        match result {
            Ok(Some(m)) => Ok(m),
            _ => Err(DbErr::RecordNotFound("User not found".into())),
        }
    }
}

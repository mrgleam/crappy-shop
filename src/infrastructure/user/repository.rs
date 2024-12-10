use std::sync::Arc;

use entity::user;
use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, InsertResult, QueryFilter,
    UpdateResult,
};

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

    pub async fn save(
        &self,
        user: &User,
    ) -> Result<InsertResult<entity::user::ActiveModel>, DbErr> {
        let model = user::ActiveModel {
            id: ActiveValue::NotSet,
            email: ActiveValue::Set(user.email.clone()),
            password: ActiveValue::Set(user.password.clone()),
            created_at: ActiveValue::Set(user.created_at.into()),
            updated_at: ActiveValue::Set(user.updated_at.into()),
        };
        user::Entity::insert(model).exec(self.db.as_ref()).await
    }

    pub async fn update(&self, user: &User) -> Result<UpdateResult, DbErr> {
        let model = user::ActiveModel {
            id: user.id.map(ActiveValue::Set).unwrap_or(ActiveValue::NotSet),
            email: ActiveValue::Set(user.email.clone()),
            updated_at: ActiveValue::Set(user.updated_at.into()),
            password: ActiveValue::Set(user.password.clone()),
            created_at: ActiveValue::Set(user.created_at.into()),
        };
        user::Entity::update(model).exec(self.db.as_ref()).await?;
        Ok(UpdateResult { rows_affected: 1 })
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

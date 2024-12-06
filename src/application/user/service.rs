use crate::{
    domain::user::{aggregate::User, error::UserError},
    infrastructure::user::repository::UserRepository,
};

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    pub fn encrypted_password<'a>(&self, user: &'a mut User) -> Result<&'a mut User, UserError> {
        let encrypted = bcrypt::hash(user.password.clone(), bcrypt::DEFAULT_COST);
        match encrypted {
            Ok(result) => {
                user.password = result;
                Ok(user)
            }
            Err(e) => Err(UserError::from(e.to_string().as_str())),
        }
    }

    pub async fn create(&self, user: &User) -> Result<i32, UserError> {
        let created = self.repository.save(&user).await;
        match created {
            Ok(result) => Ok(result.last_insert_id),
            Err(e) => Err(UserError::from(e.to_string().as_str())),
        }
    }

    pub async fn update(&self, user: &User) -> Result<u64, UserError> {
        let updated = self.repository.update(&user).await;
        match updated {
            Ok(result) => Ok(result.rows_affected),
            Err(e) => Err(UserError::from(e.to_string().as_str())),
        }
    }
}

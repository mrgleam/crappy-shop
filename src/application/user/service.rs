use crate::{
    domain::user::{aggregate::User, error::UserError, view::UserView},
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
            Err(e) => Err(UserError::from(e)),
        }
    }

    pub async fn get_by_id(&self, id: i32) -> Result<UserView, UserError> {
        let user = self.repository.find_by_id(id).await;
        match user {
            Ok(user) => Ok(UserView {
                id: Some(user.id.to_string()),
                email: user.email,
                token: None,
                created_at: Some(user.created_at.to_utc()),
                updated_at: Some(user.updated_at.to_utc()),
            }),
            Err(e) => Err(UserError::from(e)),
        }
    }

    pub async fn create(&self, user: &User) -> Result<i32, UserError> {
        let created = self.repository.save(&user).await;
        match created {
            Ok(result) => Ok(result.last_insert_id),
            Err(e) => Err(UserError::from(e)),
        }
    }

    pub async fn update(&self, user: &User) -> Result<u64, UserError> {
        let updated = self.repository.update(&user).await;
        match updated {
            Ok(result) => Ok(result.rows_affected),
            Err(e) => Err(UserError::from(e)),
        }
    }

    pub async fn signin(&self, email: String, password: String) -> Result<bool, UserError> {
        let user = self.repository.find_by_email(email).await;
        match user {
            Ok(user) => bcrypt::verify(&password, &user.password).map_err(|e| UserError::from(e)),
            Err(e) => Err(UserError::from(e)),
        }
    }
}

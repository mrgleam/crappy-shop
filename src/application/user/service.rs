use std::sync::Arc;

use serde::Serialize;

use crate::{
    config::AuthenticationConfig,
    domain::user::{aggregate::User, error::UserError, view::UserView},
    infrastructure::user::repository::UserRepository,
};

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    fn new(user_id: i32, expired_time: i64) -> Self {
        Self {
            sub: user_id.to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::minutes(expired_time)).timestamp()
                as usize,
        }
    }
}

pub struct UserService {
    repository: UserRepository,
    authentication_config: Arc<AuthenticationConfig>,
}

impl UserService {
    pub fn new(
        repository: UserRepository,
        authentication_config: Arc<AuthenticationConfig>,
    ) -> Self {
        Self {
            repository,
            authentication_config,
        }
    }
    pub fn create_token(&self, user_id: i32) -> Result<String, UserError> {
        let claims = Claims::new(user_id, self.authentication_config.expired_time);

        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(self.authentication_config.secret.as_ref()),
        );
        match token {
            Ok(token) => Ok(token),
            Err(e) => Err(UserError::from(e)),
        }
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

    pub async fn save(&self, user: User) -> Result<User, UserError> {
        let created = self.repository.save(user).await;
        match created {
            Ok(result) => Ok(result.into()),
            Err(e) => Err(UserError::from(e)),
        }
    }

    pub async fn signin(&self, email: &str, password: &str) -> Result<i32, UserError> {
        let user = self.repository.find_by_email(email).await;
        match user {
            Ok(user) => bcrypt::verify(password, &user.password)
                .map(|is_valid| self.is_valid_return_user_id(is_valid, user.id))
                .map_err(|e| UserError::from(e))?,
            Err(e) => Err(UserError::from(e)),
        }
    }

    fn is_valid_return_user_id(&self, is_valid: bool, user_id: i32) -> Result<i32, UserError> {
        if is_valid {
            Ok(user_id)
        } else {
            Err(UserError::from(UserError::VerificationFailed(
                "Invalid email or password".to_string(),
            )))
        }
    }
}

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdateUser {
    pub id: i32,
    pub email: String,
    pub password: String,
}

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct UserCreate {
    pub username: String,
    pub email: String,
    pub hashed_password: String,
}

#[derive(Serialize, ToSchema)]
pub struct UserResponse {
    pub id: u64,
    pub username: String,
    pub email: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UserUpdate {
    pub username: String,
    pub hashed_password: String,
}

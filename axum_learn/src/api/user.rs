use crate::models::user::Model as User;

pub async fn create_user(user: User) -> Result<User, String> {
    Ok(user)
}

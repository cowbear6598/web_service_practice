use serde::{Deserialize, Serialize};

use crate::user::entities::user::User;

#[derive(Serialize, Deserialize)]
pub struct UserResponseDto {
    pub uid: String,
    pub name: String,
    pub email: String,
}

impl From<User> for UserResponseDto {
    fn from(user: User) -> Self {
        Self {
            uid: user.uid,
            name: user.name,
            email: user.email,
        }
    }
}

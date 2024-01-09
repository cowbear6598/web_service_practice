use serde::{Deserialize, Serialize};

use crate::user::entities::user::User;

#[derive(Serialize, Deserialize)]
pub struct UserResponseDto {
    pub uid: String,
    pub name: String,
    pub email: String,
}

impl From<User> for UserResponseDto {
    fn from(value: User) -> Self {
        Self {
            uid: value.uid,
            name: value.name,
            email: value.email,
        }
    }
}
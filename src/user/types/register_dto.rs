use anyhow::anyhow;
use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::user::entities::user::User;
use crate::user::error::user_error::UserError;

#[derive(Serialize, Deserialize)]
pub struct RegisterDto {
    pub email: String,
    pub password: String,
    pub name: String,
}

impl TryFrom<RegisterDto> for User {
    type Error = anyhow::Error;

    fn try_from(value: RegisterDto) -> Result<Self, Self::Error> {
        let uid = Uuid::new().to_string();
        let hash_password = bcrypt::hash(value.password, bcrypt::DEFAULT_COST)
            .map_err(|_| anyhow!(UserError::PasswordNotMatch))?;

        Ok(User {
            uid,
            name: value.name,
            email: value.email,
            password: hash_password,
        })
    }
}
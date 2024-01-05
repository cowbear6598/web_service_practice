use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub uid: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn verify_password(&self, password: &str) -> Result<bool> {
        Ok(bcrypt::verify(password, &self.password)?)
    }
}
use crate::{
    adapters::user_trait::{UserRepositoryTrait, UserUseCaseTrait},
    entities::user_entity::User,
    frameworks::errors::user_error::UserError,
};
use anyhow::{Result};
use async_trait::async_trait;
use bcrypt::DEFAULT_COST;
use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

pub struct UserUseCase {
    pub repo: Box<dyn UserRepositoryTrait>,
}

impl UserUseCase {
    pub fn new(repo: Box<dyn UserRepositoryTrait>) -> UserUseCase {
        UserUseCase {
            repo
        }
    }
}

#[async_trait]
impl UserUseCaseTrait for UserUseCase {
    async fn add_user(&self, insert_data: AddUserData) -> Result<()> {
        let timestamp = chrono::Utc::now().timestamp().to_string();

        let hash_password = bcrypt::hash(&insert_data.user_password, DEFAULT_COST)
            .map_err(|_| UserError::PasswordHashFail)?;

        let user = User {
            user_id: Uuid::new().to_string(),
            user_name: insert_data.user_name,
            user_email: insert_data.user_email,
            user_password: hash_password,
            user_role: "user".to_string(),
            created_at: timestamp.clone(),
            last_login_time: timestamp,
        };

        self.repo.add_user(&user).await
    }

    async fn remove_user(&self, user_id: String) -> Result<()> {
        self.repo.remove_user(user_id).await
    }
}

#[derive(Serialize, Deserialize)]
pub struct AddUserData {
    pub user_name: String,
    pub user_email: String,
    pub user_password: String,
}

use crate::{
    adapters::user_trait::{UserRepositoryTrait, UserUseCaseTrait},
    entities::user_entity::User,
    frameworks::errors::user_error::UserError,
    frameworks::repositories::user_repository::UserRepository,
    use_cases::factory_trait::UseCaseFactory,
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bcrypt::DEFAULT_COST;
use mongodb::bson::Uuid;
use mongodb::Client;
use serde::{Deserialize, Serialize};
use shaku::Component;

#[derive(Component)]
#[shaku(interface = UserUseCaseTrait)]
pub struct UserUseCase {
    pub repo: Box<dyn UserRepositoryTrait>,
}

#[async_trait]
impl UserUseCaseTrait for UserUseCase {
    async fn add_user(&self, insert_data: AddUserData) -> Result<()> {
        let user = User::try_from(insert_data)?;

        self.repo.add_user(user).await
    }

    async fn upload_avatar(&self, user_id: String, avatar_url: String) -> Result<()> {
        self.repo.upload_avatar(user_id, avatar_url).await
    }

    async fn remove_user(&self, user_id: String) -> Result<()> {
        self.repo.remove_user(user_id).await
    }
}

impl UseCaseFactory for UserUseCase {
    type Repo = Box<dyn UserRepositoryTrait>;
    type UseCase = UserUseCaseParameters;

    fn new_repo(client: &Client) -> Self::Repo {
        Box::new(UserRepository::new(client))
    }

    fn new_use_case(repo: Self::Repo) -> Self::UseCase {
        UserUseCaseParameters {
            repo
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AddUserData {
    pub user_name: String,
    pub user_email: String,
    pub user_password: String,
}

impl TryFrom<AddUserData> for User {
    type Error = anyhow::Error;

    fn try_from(value: AddUserData) -> Result<Self, Self::Error> {
        let timestamp = chrono::Utc::now().timestamp().to_string();

        let hash_password = match bcrypt::hash(&value.user_password, DEFAULT_COST) {
            Ok(hash) => hash,
            Err(_) => return Err(anyhow!(UserError::PasswordHashFail)),
        };

        let user = User {
            user_id: Uuid::new().to_string(),
            user_name: value.user_name,
            user_email: value.user_email,
            avatar_url: "".to_string(),
            user_password: hash_password,
            user_role: "user".to_string(),
            created_at: timestamp.clone(),
            last_login_time: timestamp,
        };

        Ok(user)
    }
}
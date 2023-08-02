use async_trait::async_trait;
use anyhow::Result;
use crate::{
    entities::user_entity::User,
    use_cases::user_use_case::AddUserData,
};

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn add_user(&self, user: &User) -> Result<()>;
    async fn remove_user(&self, user_id: String) -> Result<()>;
}

#[async_trait]
pub trait UserUseCaseTrait: Send + Sync {
    async fn add_user(&self, user: AddUserData) -> Result<()>;
    async fn remove_user(&self, user_id: String) -> Result<()>;
}
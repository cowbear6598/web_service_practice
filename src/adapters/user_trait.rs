use async_trait::async_trait;
use anyhow::Result;
use crate::entities::user::User;

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn add_user(&self, user: &User) -> Result<()>;
}

#[async_trait]
pub trait UserUseCaseTrait: Send + Sync {
    async fn add_user(&self, user: &User) -> Result<()>;
}
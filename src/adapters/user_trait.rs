use async_trait::async_trait;
use anyhow::Result;
use crate::entities::user::User;

#[async_trait]
pub trait UserRepositoryTrait {
    async fn add_user(&self, user: &User) -> Result<()>;
}
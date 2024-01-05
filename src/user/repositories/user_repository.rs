use anyhow::Result;
use async_trait::async_trait;

use crate::user::entities::user::User;

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn register(&self, user: User) -> Result<()>;
}
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

use crate::{
    user::entities::user::User,
    user::types::find_dto::FindDto,
};

#[automock]
#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn find(&self, dto: &FindDto) -> Result<User>;

    async fn register(&self, user: User) -> Result<()>;

    async fn remove(&self, email: String) -> Result<()>;
}
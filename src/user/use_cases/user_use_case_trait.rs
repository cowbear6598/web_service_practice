use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

use crate::user::types::register_dto::RegisterDto;

#[automock]
#[async_trait]
pub trait UserUseCaseTrait: Send + Sync {
    async fn register(&self, register_dto: RegisterDto) -> Result<()>;
    async fn remove(&self, email: String) -> Result<()>;
}
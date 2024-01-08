use anyhow::Result;
use async_trait::async_trait;

use web_service_pratice::{
    user::entities::user::User,
    user::repositories::user_repository::UserRepositoryTrait,
    user::use_cases::user_use_case_trait::UserUseCaseTrait,
};
use web_service_pratice::user::types::register_dto::RegisterDto;

pub struct MockUserRepositoryTrait;

#[async_trait]
impl UserRepositoryTrait for MockUserRepositoryTrait {
    async fn register(&self, _: User) -> Result<()> {
        Ok(())
    }
}

pub struct MockUserUseCaseTrait;

#[async_trait]
impl UserUseCaseTrait for MockUserUseCaseTrait {
    async fn register(&self, _: RegisterDto) -> Result<()> {
        Ok(())
    }
}
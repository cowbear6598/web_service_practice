use web_service_pratice::{
    adapters::user_trait::UserRepositoryTrait,
    entities::user_entity::User,
    adapters::user_trait::UserUseCaseTrait,
    use_cases::user_use_case::AddUserData
};
use anyhow::Result;
use async_trait::async_trait;

#[cfg(test)]
pub struct MockUserRepository;

#[async_trait]
impl UserRepositoryTrait for MockUserRepository {
    async fn add_user(&self, _: User) -> Result<()> {
        Ok(())
    }
    async fn remove_user(&self, _: String) -> Result<()> { Ok(()) }
}

#[cfg(test)]
pub struct MockUserUseCase;

#[async_trait]
impl UserUseCaseTrait for MockUserUseCase {
    async fn add_user(&self, _: AddUserData) -> Result<()> {
        Ok(())
    }
    async fn remove_user(&self, _: String) -> Result<()> { Ok(()) }
}
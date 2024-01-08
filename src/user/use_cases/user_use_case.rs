use anyhow::Result;
use async_trait::async_trait;

use crate::{
    user::entities::user::User,
    user::repositories::user_repository::UserRepositoryTrait,
    user::types::register_dto::RegisterDto,
    user::use_cases::user_use_case_trait::UserUseCaseTrait,
};

pub struct UserUseCase {
    pub repo: Box<dyn UserRepositoryTrait>,
}

#[async_trait]
impl UserUseCaseTrait for UserUseCase {
    async fn register(&self, register_dto: RegisterDto) -> Result<()> {
        let user = User::try_from(register_dto)?;

        self.repo.register(user).await
    }

    async fn remove(&self, email: String) -> Result<()> {
        self.repo.remove(email).await
    }
}

impl UserUseCase {
    pub fn new(repo: Box<dyn UserRepositoryTrait>) -> Box<Self> {
        Box::new(Self { repo })
    }
}
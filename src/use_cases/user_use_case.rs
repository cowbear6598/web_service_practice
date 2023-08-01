use crate::{
    adapters::user_trait::{UserRepositoryTrait, UserUseCaseTrait},
    entities::user::User,
};
use anyhow::Result;
use async_trait::async_trait;

pub struct UserUseCase {
    pub user_repo: Box<dyn UserRepositoryTrait>,
}

impl UserUseCase {
    pub fn new(user_repo: Box<dyn UserRepositoryTrait>) -> UserUseCase {
        UserUseCase {
            user_repo
        }
    }
}

#[async_trait]
impl UserUseCaseTrait for UserUseCase {
    async fn add_user(&self, user: &User) -> Result<()> {
        self.user_repo.add_user(user).await
    }
}
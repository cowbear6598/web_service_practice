use std::sync::Arc;
use crate::{
    adapters::user_trait::UserUseCaseTrait,
    use_cases::user_use_case::AddUserData,
    adapters::user_trait::UserRepositoryTrait,
    frameworks::repositories::user_repository::UserRepository,
    frameworks::services::factory_trait::ServiceFactoryTrait,
    use_cases::user_use_case::UserUseCase,
};
use anyhow::Result;
use mongodb::Client;
use serde::{Deserialize, Serialize};

pub struct UserService {
    use_case: Box<dyn UserUseCaseTrait>,
}

impl UserService {
    pub fn new(use_case: Box<dyn UserUseCaseTrait>) -> Arc<Self> {
        Arc::new(Self {
            use_case
        })
    }

    pub async fn add_user(&self, req: AddUserRequest) -> Result<()> {
        let data = AddUserData::from(req);

        self.use_case.add_user(data).await
    }

    pub async fn upload_avatar(&self) -> Result<()> {
        todo!()
    }

    pub async fn remove_user(&self, req: RemoveUserRequest) -> Result<()> {
        self.use_case.remove_user(req.user_id).await
    }
}

impl ServiceFactoryTrait for UserService {
    type Repository = Box<dyn UserRepositoryTrait>;
    type UseCase = Box<dyn UserUseCaseTrait>;
    type Service = Arc<Self>;

    fn new_repo(client: &Client) -> Self::Repository {
        Box::new(UserRepository::new(client))
    }

    fn new_use_case(repo: Self::Repository) -> Self::UseCase {
        Box::new(UserUseCase::new(repo))
    }

    fn new_service(use_case: Self::UseCase) -> Self::Service {
        Self::new(use_case)
    }
}

#[derive(Deserialize, Serialize)]
pub struct AddUserRequest {
    pub user_email: String,
    pub user_name: String,
    pub user_password: String,
}

impl From<AddUserRequest> for AddUserData {
    fn from(value: AddUserRequest) -> Self {
        AddUserData {
            user_email: value.user_email,
            user_name: value.user_name,
            user_password: value.user_password,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct RemoveUserRequest {
    pub user_id: String,
}
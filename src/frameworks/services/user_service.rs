use std::sync::Arc;
use crate::{
    adapters::user_trait::UserUseCaseTrait,
    use_cases::user_use_case::AddUserData,
};
use anyhow::Result;
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
        let insert_data = AddUserData {
            user_email: req.user_email,
            user_name: req.user_name,
            user_password: req.user_password,
        };

        self.use_case.add_user(insert_data).await
    }
}

#[derive(Deserialize, Serialize)]
pub struct AddUserRequest {
    pub user_email: String,
    pub user_name: String,
    pub user_password: String,
}
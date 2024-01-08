use std::sync::Arc;

use crate::{
    mongo::entities::mongo_client::MongoClient,
    user::repositories::mongo_user_repository::MongoUserRepository,
    user::use_cases::user_use_case::UserUseCase,
    user::use_cases::user_use_case_trait::UserUseCaseTrait,
};

pub struct Container {
    pub mongo_client: MongoClient,
    pub user_use_case: Box<dyn UserUseCaseTrait>,
}

pub async fn create_container() -> Arc<Container> {
    let mongo_client = MongoClient::connect().await;

    let user_repo = MongoUserRepository::new(&mongo_client.client);
    let user_use_case = UserUseCase::new(user_repo);

    let container = Container {
        mongo_client,
        user_use_case,
    };

    Arc::new(container)
}
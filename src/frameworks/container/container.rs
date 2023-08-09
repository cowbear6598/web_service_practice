use std::env;
use std::sync::Arc;
use shaku::module;
use crate::{
    frameworks::mongo::mongo_client::mongo_connect,
    use_cases::user_use_case::UserUseCase,
    use_cases::factory_trait::get_use_case,
    frameworks::google::cloud_storage::CloudStorage,
    frameworks::mongo::mongo_client::MongoClient,
    frameworks::google::cloud_storage::cloud_storage_connect,
};
use crate::use_cases::claims_use_case::{ClaimsUseCase, ClaimsUseCaseParameters};

module! {
    pub Container {
        components = [
            MongoClient,
            CloudStorage,
            UserUseCase,
        ],
        providers = []
    }
}

pub async fn build_container() -> Arc<Container> {
    let mongo_client = mongo_connect().await;
    let cloud_storage = cloud_storage_connect().await;
    let claims_use_case = ClaimsUseCaseParameters::new();

    let user_use_case = get_use_case::<UserUseCase>(&mongo_client.client);

    let container = Container::builder()
        .with_component_parameters::<MongoClient>(mongo_client)
        .with_component_parameters::<CloudStorage>(cloud_storage)
        .with_component_parameters::<UserUseCase>(user_use_case)
        .with_component_parameters::<ClaimsUseCase>(claims_use_case)
        .build();

    Arc::new(container)
}
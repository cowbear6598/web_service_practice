use std::sync::Arc;

use crate::mongo::entities::mongo_client::MongoClient;

pub struct Container {
    pub mongo_client: MongoClient,
}

pub async fn create_container() -> Arc<Container> {
    let mongo_client = MongoClient::connect().await;

    let container = Container {
        mongo_client,
    };

    Arc::new(container)
}
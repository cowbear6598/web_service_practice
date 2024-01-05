#[cfg(test)]
mod mongo_tests {
    use web_service_pratice::mongo::entities::mongo_client::MongoClient;

    #[actix_rt::test]
    async fn test_mongo_connect() {
        dotenv::dotenv().ok();

        let mongo_client = MongoClient::connect().await;

        assert_ne!(mongo_client.client.list_database_names(None, None).await.unwrap().len(), 0);
    }
}
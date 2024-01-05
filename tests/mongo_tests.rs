#[cfg(test)]
mod mongo_tests {
    use web_service_pratice::mongo::mongo_client::mongo_connect;

    #[actix_rt::test]
    async fn test_mongo_connect() {
        dotenv::dotenv().ok();

        let mongo_client = mongo_connect().await;

        assert_ne!(mongo_client.client.list_database_names(None, None).await.unwrap().len(), 0);
    }
}
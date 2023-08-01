#[cfg(test)]
mod mongo_client_tests {
    use web_service_pratice::frameworks::mongo::mongo_client::mongo_connect;

    #[actix_web::test]
    #[test]
    async fn should_connect_to_mongo_success() {
        let client = mongo_connect().await;
        assert!(client.list_database_names(None, None).await.is_ok());
    }
}
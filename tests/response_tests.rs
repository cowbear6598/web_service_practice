mod response_tests {
    use actix_http::body::to_bytes;
    use serde_json::json;
    use web_service_pratice::{
        response::response_empty::response_empty,
        response::response_error::response_error
    };

    #[actix_rt::test]
    async fn test_response_empty_json_correct() {
        let response = response_empty();

        assert_eq!(200, response.status().as_u16());

        let bytes = to_bytes(response.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(json!({
            "status": 0,
            "message": "ok"
        }).to_string(), body);
    }

    #[actix_rt::test]
    async fn test_response_error_json_correct() {
        let response = response_error(101, String::from("test"));

        assert_eq!(500, response.status().as_u16());

        let bytes = to_bytes(response.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(json!({
            "status": 101,
            "message": "test"
        }).to_string(), body);
    }
}
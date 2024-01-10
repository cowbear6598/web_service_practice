mod claims_tests {
    use web_service_pratice::{
        claims::types::role::Role,
        claims::types::token_type::TokenType,
        claims::use_cases::claims_use_case::ClaimsUseCase,
        claims::use_cases::claims_use_case_trait::ClaimsUseCaseTrait,
    };

    #[test]
    fn test_create_token_success() {
        let claims_use_case = ClaimsUseCase::new("secret".to_string());

        let token = claims_use_case.create_token("uid".to_string(), Role::User, TokenType::Access, 1).unwrap();

        let dot_count = token.matches('.').count();

        assert_eq!(2, dot_count);
    }

    #[test]
    fn test_decode_token_success() {
        let claims_use_case = ClaimsUseCase::new("secret".to_string());

        let token = claims_use_case.create_token("uid".to_string(), Role::User, TokenType::Access, 10).unwrap();

        let claims = claims_use_case.decode_token(token).unwrap();

        assert_eq!("uid", claims.sub);
        assert_eq!(Role::User, claims.role);
        assert_eq!(TokenType::Access, claims.token_type);
    }

    #[actix_rt::test]
    async fn test_decode_token_expired_failed() {
        let claims_use_case = ClaimsUseCase::new("secret".to_string());

        let token = claims_use_case.create_token("uid".to_string(), Role::User, TokenType::Access, 1).unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        let claims = claims_use_case.decode_token(token);

        assert_eq!("認證已過期", claims.err().unwrap().to_string());
    }

    #[test]
    fn test_decode_token_from_req_success() {
        use actix_web::test;

        let claims_use_case = ClaimsUseCase::new("secret".to_string());

        let token = claims_use_case.create_token("uid".to_string(), Role::User, TokenType::Access, 10).unwrap();

        let req = test::TestRequest::default()
            .insert_header(("Authorization", token))
            .to_http_request();

        let claims = claims_use_case.decode_token_from_request(&req).unwrap();

        assert_eq!("uid", claims.sub);
        assert_eq!(Role::User, claims.role);
        assert_eq!(TokenType::Access, claims.token_type);
    }

    #[test]
    fn test_decode_token_from_req_failed() {
        use actix_web::test;

        let claims_use_case = ClaimsUseCase::new("secret".to_string());

        let req = test::TestRequest::default()
            .to_http_request();

        let claims = claims_use_case.decode_token_from_request(&req);

        assert_eq!("認證無效", claims.err().unwrap().to_string());
    }
}
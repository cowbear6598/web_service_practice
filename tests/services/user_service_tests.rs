#[cfg(test)]
mod user_service_tests {
    use web_service_pratice::frameworks::services::user_service::{AddUserRequest, UserService};
    use crate::user_mocks::MockUserUseCase;

    #[actix_web::test]
    #[test]
    async fn should_add_user_success() {
        let mock_use_case = Box::new(MockUserUseCase);
        let service = UserService::new(mock_use_case);

        let req = build_add_use_request();

        assert!(service.add_user(req).await.is_ok());
    }

    fn build_add_use_request() -> AddUserRequest {
        AddUserRequest {
            user_name: "user_service".to_string(),
            user_email: "test@gmail.com".to_string(),
            user_password: "123456789".to_string(),
        }
    }
}
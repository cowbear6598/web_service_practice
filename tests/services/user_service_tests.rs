#[cfg(test)]
mod user_service_tests {
    use std::sync::Arc;
    use web_service_pratice::frameworks::services::user_service::{AddUserRequest, RemoveUserRequest, UserService};
    use crate::user_mocks::MockUserUseCase;

    fn setup() -> Arc<UserService> {
        let mock_use_case = Box::new(MockUserUseCase);
        UserService::new(mock_use_case)
    }

    #[actix_web::test]
    #[test]
    async fn should_add_user_success() {
        let service = setup();

        let req = build_add_user_request();

        assert!(service.add_user(req).await.is_ok());
    }

    #[actix_web::test]
    #[test]
    async fn should_remove_user_success() {
        let service = setup();

        let req = build_remove_user_request();

        assert!(service.remove_user(req).await.is_ok());
    }

    fn build_add_user_request() -> AddUserRequest {
        AddUserRequest {
            user_name: "user_service".to_string(),
            user_email: "test@gmail.com".to_string(),
            user_password: "123456789".to_string(),
        }
    }

    fn build_remove_user_request() -> RemoveUserRequest {
        RemoveUserRequest {
            user_id: "123456789".to_string(),
        }
    }
}
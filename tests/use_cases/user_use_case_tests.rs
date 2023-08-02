#[cfg(test)]
mod user_use_case_tests {
    use web_service_pratice::{
        adapters::user_trait::{UserUseCaseTrait},
        use_cases::user_use_case::UserUseCase,
        use_cases::user_use_case::AddUserData,
    };
    use crate::mocks::user_mocks::MockUserRepository;


    #[actix_web::test]
    #[test]
    async fn should_add_user_success() {
        let repo = Box::new(MockUserRepository);
        let use_case = UserUseCase::new(repo);

        let insert_data = build_add_user_data();

        assert!(use_case.add_user(insert_data).await.is_ok());
    }

    fn build_add_user_data() -> AddUserData {
        AddUserData {
            user_name: "use_case".to_string(),
            user_email: "use_case@gmail.com".to_string(),
            user_password: "123456789".to_string(),
        }
    }
}
#[cfg(test)]
mod user_use_case_tests {
    use web_service_pratice::{
        adapters::user_trait::{UserRepositoryTrait, UserUseCaseTrait},
        entities::user::User,
        use_cases::user_use_case::UserUseCase,
    };
    use anyhow::Result;
    use async_trait::async_trait;

    struct MockUserRepository;

    #[async_trait]
    impl UserRepositoryTrait for MockUserRepository {
        async fn add_user(&self, _: &User) -> Result<()> {
            Ok(())
        }
    }

    #[actix_web::test]
    #[test]
    async fn should_add_user_success() {
        let repo = Box::new(MockUserRepository);
        let use_case = UserUseCase::new(repo);

        let user = test_user();

        assert!(use_case.add_user(&user).await.is_ok());
    }

    fn test_user() -> User {
        User {
            user_id: "use_case".to_string(),
            user_name: "name".to_string(),
            user_email: "email".to_string(),
            user_password: "password".to_string(),
            user_role: "user".to_string(),
            created_at: "10000".to_string(),
            last_login_time: "10000".to_string(),
        }
    }
}
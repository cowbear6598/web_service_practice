#[cfg(test)]
mod user_use_case_tests {
    use web_service_pratice::{
        adapters::user_trait::{UserRepositoryTrait, UserUseCaseTrait},
        entities::user_entity::User,
        use_cases::user_use_case::UserUseCase,
    };
    use anyhow::Result;
    use async_trait::async_trait;
    use web_service_pratice::use_cases::user_use_case::AddUserData;

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
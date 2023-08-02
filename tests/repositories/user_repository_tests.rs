#[cfg(test)]
mod user_repository_tests {
    use web_service_pratice::{
        frameworks::mongo::mongo_client::mongo_connect,
        adapters::user_trait::UserRepositoryTrait,
        frameworks::repositories::user_repository::UserRepository,
    };
    use web_service_pratice::entities::user_entity::User;

    async fn setup() -> Box<dyn UserRepositoryTrait> {
        let client = mongo_connect().await;
        let repo: Box<dyn UserRepositoryTrait> = Box::new(UserRepository::new(&client));

        repo
    }

    #[actix_web::test]
    #[test]
    pub async fn should_add_user_success() {
        let repo = setup().await;
        let user = test_user();

        assert!(repo.add_user(&user).await.is_ok());
    }

    #[actix_web::test]
    #[test]
    pub async fn should_remove_user_success() {
        let repo = setup().await;
        let user = test_user();

        assert!(repo.remove_user(user.user_id).await.is_ok());
    }

    fn test_user() -> User {
        User {
            user_id: "repository".to_string(),
            user_name: "name".to_string(),
            user_email: "email".to_string(),
            user_password: "password".to_string(),
            user_role: "user".to_string(),
            created_at: "10000".to_string(),
            last_login_time: "10000".to_string(),
        }
    }
}
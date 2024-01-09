use web_service_pratice::{
    user::repositories::user_repository::MockUserRepositoryTrait,
    user::use_cases::user_use_case::UserUseCase,
    user::use_cases::user_use_case_trait::UserUseCaseTrait,
};
use web_service_pratice::user::types::find_dto::FindDto;

use crate::common::fake_data::{fake_register_dto, fake_user};

#[actix_rt::test]
async fn test_find_operation_successful() {
    let mut mock_use_repo = Box::new(MockUserRepositoryTrait::new());

    mock_use_repo.expect_find()
        .once()
        .returning(|_| Ok(fake_user()));

    let user_use_case = UserUseCase::new(mock_use_repo);

    let find_dto = FindDto {
        email: Some("test@gmail.com".to_string()),
    };

    let result = user_use_case.find(&find_dto).await.unwrap();

    assert_eq!(result.email, "test@gmail.com");
    assert_eq!(result.name, "test");
}

#[actix_rt::test]
async fn test_register_operation_successful() {
    let mut mock_user_repo = Box::new(MockUserRepositoryTrait::new());

    mock_user_repo.expect_register()
        .once()
        .returning(|_| Ok(()));

    let user_use_case = UserUseCase::new(mock_user_repo);

    let register_dto = fake_register_dto();

    let result = user_use_case.register(register_dto).await;

    assert!(result.is_ok());
}

#[actix_rt::test]
async fn test_remove_user_operation_successful() {
    let mut mock_user_repo = Box::new(MockUserRepositoryTrait::new());

    mock_user_repo.expect_remove()
        .once()
        .returning(|_| Ok(()));

    let user_use_case = UserUseCase::new(mock_user_repo);

    let email = "test".to_string();

    let result = user_use_case.remove(email).await;

    assert!(result.is_ok());
}
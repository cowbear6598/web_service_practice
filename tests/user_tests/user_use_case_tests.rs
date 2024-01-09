use web_service_pratice::{
    user::repositories::user_repository::MockUserRepositoryTrait,
    user::use_cases::user_use_case::UserUseCase,
    user::use_cases::user_use_case_trait::UserUseCaseTrait,
};

use crate::common::fake_data::fake_register_dto;

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
async fn should_remove_user_successful() {
    let mut mock_user_repo = Box::new(MockUserRepositoryTrait::new());

    mock_user_repo.expect_remove()
        .once()
        .returning(|_| Ok(()));

    let user_use_case = UserUseCase::new(mock_user_repo);

    let email = "test".to_string();

    let result = user_use_case.remove(email).await;

    assert!(result.is_ok());
}
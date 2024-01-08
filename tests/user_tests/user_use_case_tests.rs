use web_service_pratice::{
    user::use_cases::user_use_case::UserUseCase,
    user::use_cases::user_use_case_trait::UserUseCaseTrait,
};

use crate::{
    common::fake_data::fake_register_dto,
    common::mock::user_mock::MockUserRepositoryTrait,
};

#[actix_rt::test]
async fn should_register_successful() {
    let mock_user_repo = Box::new(MockUserRepositoryTrait {});

    let user_use_case = UserUseCase::new(mock_user_repo);

    let register_dto = fake_register_dto();

    let result = user_use_case.register(register_dto).await;

    assert!(result.is_ok());
}
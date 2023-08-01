use actix_web::{HttpResponse, post, web};
use mongodb::Client;
use crate::{
    frameworks::repositories::user_repository::UserRepository,
    use_cases::user_use_case::UserUseCase,
    frameworks::services::user_service::AddUserRequest,
    frameworks::services::user_service::UserService,
    frameworks::errors::to_u16_trait::ToU16,
    frameworks::errors::user_error::UserError,
    frameworks::web::response_handler::{response_message, response_ok}
};

#[post("user/add_user")]
pub async fn add_user(client: web::Data<Client>, form: web::Json<AddUserRequest>) -> HttpResponse {
    let repo = Box::new(UserRepository::new(&client));
    let use_case = Box::new(UserUseCase::new(repo));
    let service = UserService::new(use_case);

    match service.add_user(form.into_inner()).await {
        Ok(_) => response_ok(),
        Err(err) => response_message(UserError::AddUserFail.to_u16(), err.to_string()),
    }
}
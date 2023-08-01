use actix_web::{HttpResponse, post, web};
use mongodb::Client;
use crate::{
    frameworks::repositories::user_repository::UserRepository,
    use_cases::user_use_case::UserUseCase,
};
use crate::adapters::user_trait::UserUseCaseTrait;

#[post("user/add_user")]
pub async fn add_user(client: web::Data<Client>) -> HttpResponse {
    let repo = Box::new(UserRepository::new(&client));
    let use_case = UserUseCase::new(repo);

    match use_case.add_user().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
use actix_web::{HttpResponse, post, web};
use mongodb::Client;
use serde_json::json;
use crate::{
    frameworks::repositories::user_repository::UserRepository,
    use_cases::user_use_case::UserUseCase,
    frameworks::services::user_service::AddUserRequest,
    frameworks::services::user_service::UserService,
};

#[post("user/add_user")]
pub async fn add_user(client: web::Data<Client>, form: web::Json<AddUserRequest>) -> HttpResponse {
    let repo = Box::new(UserRepository::new(&client));
    let use_case = Box::new(UserUseCase::new(repo));
    let service = UserService::new(use_case);

    match service.add_user(form.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": 0,
            "message": "ok",
        })),
        Err(err) => HttpResponse::Ok().json(json!({
            "status": 4001,
            "message": err.to_string(),
        }))
    }
}
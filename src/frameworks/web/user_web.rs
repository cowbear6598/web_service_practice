use std::sync::Arc;
use actix_web::{HttpResponse, post, web};
use crate::{
    frameworks::services::user_service::AddUserRequest,
    frameworks::services::user_service::UserService,
    frameworks::web::response_handler::{response_message, response_ok},
    frameworks::services::user_service::RemoveUserRequest,
    frameworks::errors::response_code::ResponseCode,
};

#[post("user/add_user")]
pub async fn add_user(service: web::Data<Arc<UserService>>, form: web::Json<AddUserRequest>) -> HttpResponse {
    match service.add_user(form.into_inner()).await {
        Ok(_) => response_ok(),
        Err(err) => response_message(ResponseCode::AddUserFail.to_u16(), err.to_string()),
    }
}

#[post("user/remove_user")]
pub async fn remove_user(service: web::Data<Arc<UserService>>, form: web::Json<RemoveUserRequest>) -> HttpResponse {
    match service.remove_user(form.into_inner()).await {
        Ok(_) => response_ok(),
        Err(err) => response_message(ResponseCode::RemoveUserFail.to_u16(), err.to_string()),
    }
}
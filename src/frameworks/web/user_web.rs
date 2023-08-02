use std::sync::Arc;
use actix_web::{HttpResponse, post, web};
use crate::{
    frameworks::services::user_service::AddUserRequest,
    frameworks::services::user_service::UserService,
    frameworks::errors::to_u16_trait::ToU16,
    frameworks::errors::user_error::UserError,
    frameworks::web::response_handler::{response_message, response_ok},
};

#[post("user/add_user")]
pub async fn add_user(service: web::Data<Arc<UserService>>, form: web::Json<AddUserRequest>) -> HttpResponse {
    match service.add_user(form.into_inner()).await {
        Ok(_) => response_ok(),
        Err(err) => response_message(UserError::AddUserFail.to_u16(), err.to_string()),
    }
}
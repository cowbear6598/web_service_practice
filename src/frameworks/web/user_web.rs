use std::sync::Arc;
use actix_multipart::Multipart;
use actix_web::{HttpRequest, HttpResponse, post, web};
use crate::{
    frameworks::services::user_service::AddUserRequest,
    frameworks::services::user_service::UserService,
    frameworks::web::response_handler::{response_error, response_ok},
    frameworks::services::user_service::RemoveUserRequest,
    frameworks::errors::response_code::ResponseCode,
};
use crate::frameworks::common::multipart::get_field_name;

#[post("user/add_user")]
pub async fn add_user(service: web::Data<Arc<UserService>>, form: web::Json<AddUserRequest>) -> HttpResponse {
    match service.add_user(form.into_inner()).await {
        Ok(_) => response_ok(),
        Err(err) => response_error(ResponseCode::AddUserFail.to_u16(), err.to_string()),
    }
}

#[post("user/upload_avatar")]
pub async fn upload_avatar(service: web::Data<Arc<UserService>>, req: HttpRequest, mut payload: Multipart) -> HttpResponse {
    let mut avatar_url = String::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let field_name = get_field_name(&mut field)
            .map_err(|err| { return response_error(ResponseCode::UploadAvatarFail.to_u16(), err.to_string()); })?;

        match field_name.as_str() {
            "avatar_file" => avatar_url =,
            _ => return response_error(ResponseCode::UploadAvatarFail.to_u16(), "Invalid field name".to_string()),
        }
    }

    match service.upload_avatar().await {
        Ok(_) => response_ok(),
        Err(err) => response_error(ResponseCode::UploadAvatarFail.to_u16(), err.to_string()),
    }
}

#[post("user/remove_user")]
pub async fn remove_user(service: web::Data<Arc<UserService>>, form: web::Json<RemoveUserRequest>) -> HttpResponse {
    match service.remove_user(form.into_inner()).await {
        Ok(_) => response_ok(),
        Err(err) => response_error(ResponseCode::RemoveUserFail.to_u16(), err.to_string()),
    }
}
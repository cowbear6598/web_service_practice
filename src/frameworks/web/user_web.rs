use std::sync::Arc;
use anyhow::{Result};
use actix_multipart::Multipart;
use actix_web::{HttpResponse, post, web};
use serde::{Deserialize, Serialize};
use shaku::HasComponent;
use crate::{
    frameworks::web::response_handler::{response_error, response_ok},
    frameworks::errors::response_code::ResponseCode,
    use_cases::user_use_case::AddUserData,
    frameworks::container::container::Container,
    adapters::user_trait::UserUseCaseTrait,
    adapters::cloud_storage_trait::CloudStorageTrait,
    frameworks::multipart::multipart_handler::{Method, MultipartHandler},
    adapters::hashmap_trait::HashmapExtensionTrait,
};

#[post("user/add_user")]
pub async fn add_user(container: web::Data<Arc<Container>>, form: web::Json<AddUserRequest>) -> HttpResponse {
    let use_case: &dyn UserUseCaseTrait = container.resolve_ref();
    let req_data = form.into_inner();

    let insert_data = AddUserData::from(req_data);

    match use_case.add_user(insert_data).await {
        Ok(_) => response_ok(),
        Err(err) => response_error(ResponseCode::AddUserFail.to_u16(), err.to_string()),
    }
}

#[post("user/upload_avatar")]
pub async fn upload_avatar(container: web::Data<Arc<Container>>, payload: Multipart) -> HttpResponse {
    let user_use_case: &dyn UserUseCaseTrait = container.resolve_ref();
    let cloud_storage: &dyn CloudStorageTrait = container.resolve_ref();

    let avatar_url = match handle_multipart(cloud_storage, payload).await {
        Ok(avatar_url) => avatar_url,
        Err(err) => return response_error(ResponseCode::UploadAvatarFail.to_u16(), err.to_string()),
    };

    // TODO: change real user id
    match user_use_case.upload_avatar("123".to_string(), avatar_url).await {
        Ok(_) => response_ok(),
        Err(err) => response_error(ResponseCode::UploadAvatarFail.to_u16(), err.to_string()),
    }
}

#[post("user/remove_user")]
pub async fn remove_user(container: web::Data<Arc<Container>>, form: web::Json<RemoveUserRequest>) -> HttpResponse {
    let use_case: &dyn UserUseCaseTrait = container.resolve_ref();
    let req_data = form.into_inner();

    match use_case.remove_user(req_data.user_id).await {
        Ok(_) => response_ok(),
        Err(err) => response_error(ResponseCode::RemoveUserFail.to_u16(), err.to_string()),
    }
}

async fn handle_multipart(cloud_storage: &dyn CloudStorageTrait, payload: Multipart) -> Result<String> {
    let multipart_handler = MultipartHandler::new(cloud_storage)
        .add_field("avatar_url", Method::Image, None, Some("user".to_string()));

    let mut data = multipart_handler.handle(payload).await?;

    Ok(
        data.get_result("avatar_url")?
    )
}


#[derive(Deserialize, Serialize)]
pub struct AddUserRequest {
    pub user_email: String,
    pub user_name: String,
    pub user_password: String,
}

impl From<AddUserRequest> for AddUserData {
    fn from(value: AddUserRequest) -> Self {
        AddUserData {
            user_email: value.user_email,
            user_name: value.user_name,
            user_password: value.user_password,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct RemoveUserRequest {
    pub user_id: String,
}
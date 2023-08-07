use std::sync::Arc;
use actix_web::{HttpResponse, post, web};
use serde::{Deserialize, Serialize};
use shaku::HasComponent;
use crate::{
    frameworks::web::response_handler::{response_error, response_ok},
    frameworks::errors::response_code::ResponseCode,
    use_cases::user_use_case::AddUserData,
    frameworks::container::container::Container,
    adapters::user_trait::UserUseCaseTrait,
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

// #[post("user/upload_avatar")]
// pub async fn upload_avatar(container: web::Data<Arc<Container>>, cloud_storage: web::Data<CloudStorage>,
//                            req: HttpRequest, mut payload: Multipart) -> HttpResponse
// {
//     let mut avatar_url = String::new();
//
//     while let Ok(Some(mut field)) = payload.try_next().await {
//         let field_name = match get_field_name(&mut field) {
//             Ok(field_name) => field_name,
//             Err(err) => return response_error(ResponseCode::UploadAvatarFail.to_u16(), err.to_string()),
//         };
//
//         match field_name.as_str() {
//             "avatar_file" => avatar_url = cloud_storage.upload_image(field).await.unwrap(),
//             _ => return response_error(ResponseCode::UploadAvatarFail.to_u16(), "Invalid field name".to_string()),
//         }
//     }
//
//     match service.upload_avatar(avatar_url).await {
//         Ok(_) => response_ok(),
//         Err(err) => response_error(ResponseCode::UploadAvatarFail.to_u16(), err.to_string()),
//     }
// }

#[post("user/remove_user")]
pub async fn remove_user(container: web::Data<Arc<Container>>, form: web::Json<RemoveUserRequest>) -> HttpResponse {
    let use_case: &dyn UserUseCaseTrait = container.resolve_ref();
    let req_data = form.into_inner();

    match use_case.remove_user(req_data.user_id).await {
        Ok(_) => response_ok(),
        Err(err) => response_error(ResponseCode::RemoveUserFail.to_u16(), err.to_string()),
    }
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
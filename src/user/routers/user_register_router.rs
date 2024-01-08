use std::sync::Arc;

use actix_web::{HttpResponse, post};
use actix_web::web::{Data, Json};

use crate::{
    container::container::Container,
    response::response_empty::response_empty,
    response::response_error::response_error,
    user::types::register_dto::RegisterDto,
};

#[post("user/register")]
pub async fn execute(container: Data<Arc<Container>>, form: Json<RegisterDto>) -> HttpResponse {
    let user_use_case = &container.user_use_case;

    let req_data = form.into_inner();

    match user_use_case.register(req_data).await {
        Ok(_) => response_empty(),
        Err(err) => response_error(101, err.to_string()),
    }
}
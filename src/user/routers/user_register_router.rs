use std::sync::Arc;

use actix_web::{HttpResponse, post};
use actix_web::web::{Data, Json};

use crate::{
    container::container::Container,
    user::types::register_dto::RegisterDto,
};

#[post("user/register")]
pub async fn execute(container: Data<Arc<Container>>, form: Json<RegisterDto>) -> HttpResponse {
    let user_use_case = &container.user_use_case;

    let req_data = form.into_inner();

    match user_use_case.register(req_data) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}
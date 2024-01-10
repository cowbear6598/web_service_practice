use std::sync::Arc;

use actix_web::{get, HttpResponse};
use actix_web::web::{Data, Path};

use crate::{
    container::container::Container,
    response::response_data::response_data,
    response::response_error::response_error,
    user::types::find_dto::FindDto,
};

#[get("/user/find_by_email/{email}")]
pub async fn execute(container: Data<Arc<Container>>, email: Path<String>) -> HttpResponse {
    let user_use_case = &container.user_use_case;

    let email = email.into_inner();
    let dto = FindDto {
        email: Some(email),
    };

    match user_use_case.find(&dto).await {
        Ok(user) => response_data(user),
        Err(err) => response_error(101, err.to_string()),
    }
}
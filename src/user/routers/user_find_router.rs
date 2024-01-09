use std::sync::Arc;

use actix_web::{get, HttpResponse};
use actix_web::web::{Data, Query};

use crate::{
    container::container::Container,
    response::response_data::response_data,
    response::response_error::response_error,
    user::types::find_dto::FindDto,
};

#[get("/user/find")]
pub async fn execute(container: Data<Arc<Container>>, form: Query<FindDto>) -> HttpResponse {
    let use_case = &container.user_use_case;

    let req_data = form.into_inner();

    match use_case.find(&req_data).await {
        Ok(user) => response_data(user),
        Err(err) => response_error(101, err.to_string())
    }
}
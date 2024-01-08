use actix_web::{HttpResponse, post};

#[post("user/register")]
pub async fn execute() -> HttpResponse {
    HttpResponse::Ok().finish()
}
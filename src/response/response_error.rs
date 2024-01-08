use actix_web::HttpResponse;
use serde_json::json;

pub fn response_error(status: u16, message: String) -> HttpResponse {
    HttpResponse::InternalServerError().json(json!({
        "status": status,
        "message": message
    }))
}
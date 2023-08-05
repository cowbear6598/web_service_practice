use actix_web::HttpResponse;
use serde_json::json;

pub fn response_ok() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": 0,
        "message": "ok",
    }))
}

pub fn response_error(status: u16, message: String) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": status,
        "message": message,
    }))
}
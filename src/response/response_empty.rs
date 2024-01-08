use actix_web::HttpResponse;
use serde_json::json;

pub fn response_empty() -> HttpResponse {
    HttpResponse::Ok().json(json!(
        {
            "status": 0,
            "message": "ok"
        }
    ))
}
use actix_web::HttpResponse;
use serde::Serialize;
use serde_json::json;

pub fn response_data<T: Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": 0,
        "message": "ok",
        "data": data
    }))
}
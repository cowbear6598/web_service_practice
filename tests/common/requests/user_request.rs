use actix_http::Request;
use actix_web::test;
use crate::common::fake_data::fake_register_dto;

pub fn create_register_request() -> Request {
    let req = test::TestRequest::post()
        .uri("/user/register")
        .set_json(fake_register_dto())
        .to_request();

    req
}
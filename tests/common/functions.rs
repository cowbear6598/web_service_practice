use actix_web::dev::ServiceResponse;
use actix_web::test;

pub async fn response_to_string(response: ServiceResponse) -> String {
    let bytes = test::read_body(response).await;
    let body = String::from_utf8(bytes.to_vec()).unwrap();

    body
}
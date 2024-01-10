use std::sync::Arc;
use actix_web::{
    App,
    test,
    web::Data,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serial_test::serial;

use web_service_pratice::{
    container::container::{Container, create_container},
    user::types::user_response_dto::UserResponseDto,
};

use crate::{
    common::functions::response_to_string,
    common::requests::user_request::create_register_request,
    common::requests::user_request::create_find_query_request
};

#[actix_rt::test]
#[serial]
async fn test_find_by_email_success() {
    let container = setup().await;

    let app = test::init_service(
        App::new()
            .app_data(Data::new(container.clone()))
            .service(web_service_pratice::user::routers::register)
            .service(web_service_pratice::user::routers::find_by_email)
    ).await;

    let req = create_register_request();
    let _ = test::call_service(&app, req).await;
    let req = test::TestRequest::get()
        .uri("/user/find_by_email/test@gmail.com")
        .to_request();

    let response = test::call_service(&app, req).await;

    assert_eq!(200, response.status().as_u16());

    let body = response_to_string(response).await;
    let user = serde_json::from_str::<UserDtoResponse>(&body).unwrap();

    assert_eq!("test@gmail.com", user.data.email);
    assert_eq!("test", user.data.name);
}

#[actix_rt::test]
#[serial]
async fn test_find_success() {
    let container = setup().await;

    let app = test::init_service(
        App::new()
            .app_data(Data::new(container.clone()))
            .service(web_service_pratice::user::routers::register)
            .service(web_service_pratice::user::routers::find)
    ).await;

    let req = create_register_request();
    let _ = test::call_service(&app, req).await;
    let req = create_find_query_request();

    let response = test::call_service(&app, req).await;

    assert_eq!(200, response.status().as_u16());

    let body = response_to_string(response).await;
    let user = serde_json::from_str::<UserDtoResponse>(&body).unwrap();

    assert_eq!("test@gmail.com", user.data.email);
    assert_eq!("test", user.data.name);

    delete_test_user(container).await;
}

#[actix_rt::test]
#[serial]
async fn test_register_success() {
    let container = setup().await;

    let app = test::init_service(
        App::new()
            .app_data(Data::new(container.clone()))
            .service(web_service_pratice::user::routers::register)
    ).await;

    let req = create_register_request();

    let response = test::call_service(&app, req).await;

    let body = response_to_string(response).await;

    assert_eq!(json!({
        "status":0,
        "message":"ok"}
        ).to_string(), body);

    delete_test_user(container).await;
}

async fn delete_test_user(container: Arc<Container>) {
    container.user_use_case.remove("test@gmail.com".to_string()).await.expect("移除使用者失敗");
}

async fn setup() -> Arc<Container> {
    dotenv::dotenv().ok();
    let container = create_container().await;
    container
}

#[derive(Serialize, Deserialize)]
struct UserDtoResponse {
    status: u16,
    message: String,
    data: UserResponseDto,
}
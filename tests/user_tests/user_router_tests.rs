use std::sync::Arc;

use actix_web::{App, test};
use actix_web::dev::ServiceResponse;
use actix_web::web::Data;
use serde_json::json;

use web_service_pratice::{
    container::container::Container,
    user::use_cases::user_use_case_trait::{MockUserUseCaseTrait, UserUseCaseTrait},
};

use crate::{
    common::functions::response_to_string,
    common::requests::user_request::create_register_request,
    common::fake_data::fake_user_response_dto
};

#[actix_rt::test]
async fn test_find_by_email_success() {
    let container = create_find_container();

    let app = test::init_service(
        App::new()
            .app_data(Data::new(container.clone()))
            .service(web_service_pratice::user::routers::register)
            .service(web_service_pratice::user::routers::find_by_email)
    ).await;

    let req = create_register_request();
    let _ = test::call_service(&app, req).await;

    let req = test::TestRequest::default()
        .uri("/user/find_by_email/test@gmail.com")
        .to_request();

    let response = test::call_service(&app, req).await;

    assert_eq!(200, response.status().as_u16());

    should_equal_test_user(response).await;
}

#[actix_rt::test]
async fn test_find_success() {
    let container = create_find_container();

    let app = test::init_service(
        App::new()
            .app_data(Data::new(container.clone()))
            .service(web_service_pratice::user::routers::register)
            .service(web_service_pratice::user::routers::find)
    ).await;

    let req = create_register_request();
    let _ = test::call_service(&app, req).await;

    let req = test::TestRequest::default()
        .uri("/user/find")
        .to_request();

    let response = test::call_service(&app, req).await;

    assert_eq!(200, response.status().as_u16());

    should_equal_test_user(response).await;
}

#[actix_rt::test]
async fn test_register_success() {
    let mut mock_user_use_case = Box::new(
        MockUserUseCaseTrait::new()
    );

    mock_user_use_case.expect_register()
        .once()
        .returning(|_| Ok(()));

    let container = create_container(mock_user_use_case);

    let app = test::init_service(
        App::new()
            .app_data(Data::new(container.clone()))
            .service(web_service_pratice::user::routers::register)
    ).await;

    let req = create_register_request();

    let response = test::call_service(&app, req).await;

    assert_eq!(200, response.status().as_u16());

    let body = response_to_string(response).await;

    assert_eq!(json!({
        "status": 0,
        "message": "ok"
    }).to_string(), body);
}

async fn should_equal_test_user(response: ServiceResponse) {
    let body = response_to_string(response).await;

    assert_eq!(json!({
        "status": 0,
        "message": "ok",
        "data": {
            "uid": "1",
            "name": "test",
            "email": "test@gmail.com"
        }
    }).to_string(), body);
}

fn create_find_container() -> Arc<Container> {
    let mut mock_user_use_case = Box::new(
        MockUserUseCaseTrait::new()
    );

    mock_user_use_case.expect_register()
        .once()
        .returning(|_| Ok(()));

    mock_user_use_case.expect_find()
        .once()
        .returning(|_| Ok(fake_user_response_dto()));

    let container = create_container(mock_user_use_case);
    container
}

fn create_container(user_use_case: Box<dyn UserUseCaseTrait>) -> Arc<Container> {
    Arc::new(
        Container {
            user_use_case,
        }
    )
}
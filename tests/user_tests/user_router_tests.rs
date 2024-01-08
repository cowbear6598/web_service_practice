use std::sync::Arc;

use actix_web::{App, test};
use actix_web::web::Data;
use serde_json::json;

use web_service_pratice::container::container::Container;

use crate::{
    common::fake_data::fake_register_dto,
    common::mock::user_mock::MockUserUseCaseTrait,
};

#[actix_rt::test]
async fn test_register_success() {
    let container = create_container();

    let app = test::init_service(
        App::new()
            .app_data(Data::new(container.clone()))
            .service(web_service_pratice::user::routers::register)
    ).await;

    let req = test::TestRequest::post()
        .uri("/user/register")
        .set_json(fake_register_dto())
        .to_request();

    let response = test::call_service(&app, req).await;

    assert_eq!(200, response.status().as_u16());

    let bytes = test::read_body(response).await;
    let body = String::from_utf8(bytes.to_vec()).unwrap();

    assert_eq!(json!({
        "status": 0,
        "message": "ok"
    }).to_string(), body);
}

fn create_container() -> Arc<Container> {
    Arc::new(
        Container {
            user_use_case: Box::new(MockUserUseCaseTrait {}),
        }
    )
}
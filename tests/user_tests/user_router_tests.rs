use std::sync::Arc;

use actix_web::{App, test};
use actix_web::web::Data;
use serde_json::json;

use web_service_pratice::container::container::Container;

use crate::{
    common::functions::response_to_string,
    common::mocks::user_mock::MockUserUseCaseTrait,
    common::requests::user_request::create_register_request,
};

#[actix_rt::test]
async fn test_register_success() {
    let container = create_container();

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

fn create_container() -> Arc<Container> {
    Arc::new(
        Container {
            user_use_case: Box::new(MockUserUseCaseTrait {}),
        }
    )
}
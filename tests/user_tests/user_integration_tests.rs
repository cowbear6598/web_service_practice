use actix_web::{App, test};
use actix_web::web::Data;
use serde_json::json;

use web_service_pratice::container::container::create_container;

use crate::{
    common::functions::response_to_string,
    common::requests::user_request::create_register_request,
};

#[actix_rt::test]
async fn test_register_success() {
    dotenv::dotenv().ok();
    let container = create_container().await;

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
}
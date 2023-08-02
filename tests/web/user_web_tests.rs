#[cfg(test)]
mod user_web_tests {
    use actix_web::{
        App,
        test::{
            TestRequest,
            init_service,
            call_and_read_body_json,
        },
        web::Data,
    };
    use serde_json::json;
    use web_service_pratice::{
        frameworks::services::user_service::RemoveUserRequest,
        frameworks::web::user_web::add_user,
        frameworks::services::user_service::UserService,
        frameworks::services::user_service::AddUserRequest,
        frameworks::web::user_web::remove_user
    };
    use crate::user_mocks::MockUserUseCase;

    async fn setup() -> impl actix_web::dev::Service<
        actix_http::Request,
        Response=actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>,
        Error=actix_web::Error,
    > {
        let use_case = Box::new(MockUserUseCase);
        let service = UserService::new(use_case);

        let app = init_service(
            App::new()
                .app_data(Data::new(service.clone()))
                .service(add_user)
                .service(remove_user)
        ).await;

        app
    }

    #[actix_web::test]
    #[test]
    async fn should_add_user_success() {
        let app = setup().await;

        let req = TestRequest::post()
            .uri("/user/add_user")
            .set_json(json!(build_add_user_request()))
            .to_request();

        let res: serde_json::Value = call_and_read_body_json(&app, req).await;

        assert_eq!(res["status"], 0);
        assert_eq!(res["message"], "ok");
    }

    #[actix_web::test]
    #[test]
    async fn should_remove_user_success() {
        let app = setup().await;

        let req = TestRequest::post()
            .uri("/user/remove_user")
            .set_json(json!(build_remove_user_request()))
            .to_request();

        let res: serde_json::Value = call_and_read_body_json(&app, req).await;

        assert_eq!(res["status"], 0);
        assert_eq!(res["message"], "ok");
    }

    fn build_add_user_request() -> AddUserRequest {
        AddUserRequest {
            user_name: "user_web".to_string(),
            user_email: "user_web@gmail.com".to_string(),
            user_password: "123456789".to_string(),
        }
    }

    fn build_remove_user_request() -> RemoveUserRequest {
        RemoveUserRequest {
            user_id: "123456789".to_string(),
        }
    }
}
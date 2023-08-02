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
        frameworks::web::user_web::add_user,
        use_cases::user_use_case::AddUserData,
        frameworks::services::user_service::UserService,
    };
    use crate::user_mocks::MockUserUseCase;

    #[actix_web::test]
    #[test]
    async fn should_add_user_success() {
        let use_case = Box::new(MockUserUseCase);
        let service = UserService::new(use_case);

        let app = init_service(
            App::new()
                .app_data(Data::new(service.clone()))
                .service(add_user)
        ).await;

        let req = TestRequest::post()
            .uri("/user/add_user")
            .set_json(json!(build_add_user_data()))
            .to_request();

        let res: serde_json::Value = call_and_read_body_json(&app, req).await;

        assert_eq!(res["status"], 0);
        assert_eq!(res["message"], "ok");
    }

    fn build_add_user_data() -> AddUserData {
        AddUserData {
            user_name: "user_web".to_string(),
            user_email: "user_web@gmail.com".to_string(),
            user_password: "123456789".to_string(),
        }
    }
}
use std::env;
use std::sync::Arc;
use actix_web::{
    App,
    HttpServer,
    web::{Data, scope},
};
use mongodb::Client;
use web_service_pratice::{
    frameworks::mongo::mongo_client::mongo_connect,
    frameworks::web,
    frameworks::repositories::user_repository::UserRepository,
    frameworks::services::user_service::UserService,
    use_cases::user_use_case::UserUseCase,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let client = mongo_connect().await;
    let user_service = get_user_service(&client);

    let (host, port) = get_address();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(user_service.clone()))
            .service(
                scope("/api/v1")
                    .service(web::user_web::add_user)
            )
    })
        .bind((host, port))?
        .run()
        .await
}

fn get_address() -> (String, u16) {
    let host = env::var("SERVER_HOST")
        .unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or_else(|_| 8080);

    (host, port)
}

fn get_user_service(client: &Client) -> Arc<UserService> {
    let repo = Box::new(UserRepository::new(client));
    let use_case = Box::new(UserUseCase::new(repo));
    let service = UserService::new(use_case);

    service
}
use std::env;
use std::sync::Arc;
use actix_web::{
    App,
    HttpServer,
    web::{Data, scope},
    middleware::Logger
};
use log::info;
use web_service_pratice::{
    frameworks::mongo::mongo_client::mongo_connect,
    frameworks::web,
    frameworks::services::user_service::UserService,
    frameworks::services::factory_trait::get_service,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_logger();

    dotenv::dotenv().ok();

    let client = mongo_connect().await;
    let user_service: Arc<UserService> = get_service::<UserService>(&client);

    let (host, port) = get_address();

    info!("伺服器啟動中: {}-{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(user_service.clone()))
            .wrap(Logger::default())
            .service(
                scope("/api/v1")
                    .service(web::user_web::add_user)
                    .service(web::user_web::remove_user)
            )
    })
        .bind((host, port))?
        .run()
        .await
}

fn set_logger() {
    log4rs::init_file("log4rs.yaml", Default::default())
        .expect("log4rs.yaml 讀取失敗");
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
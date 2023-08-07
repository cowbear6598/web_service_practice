use std::env;
use actix_web::{
    App,
    HttpServer,
    web::{Data, scope},
    middleware::Logger,
};
use log::info;
use web_service_pratice::{
    frameworks::web,
    frameworks::container::container::build_container
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    set_logger();

    let (host, port) = get_address();

    let container = build_container().await;

    // let cloud_storage = CloudStorage::new().await;

    info!("伺服器啟動中: {}-{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(container.clone()))
            // .app_data(Data::new(cloud_storage))
            .wrap(Logger::default())
            .service(
                scope("/api/v1")
                    .service(web::user_web::add_user)
                    // .service(web::user_web::upload_avatar)
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
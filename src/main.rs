use std::env;

use actix_web::{
    App,
    HttpServer,
    middleware::Logger,
    web::{Data, scope},
};
use log::info;

use web_service_pratice::container::container::create_container;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    set_logger();

    let (host, port) = get_address();

    let container = create_container().await;

    info!("伺服器啟動中: {}-{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(container.clone()))
            .wrap(Logger::default())
            .service(
                scope("/api/v1")
                    .service(web_service_pratice::user::routers::register)
                    .service(web_service_pratice::user::routers::find)
                    .service(web_service_pratice::user::routers::find_by_email)
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
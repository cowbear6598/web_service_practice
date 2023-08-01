use std::env;
use actix_web::{
    App,
    HttpServer,
    web::{Data, scope},
};
use web_service_pratice::{
    frameworks::mongo::mongo_client::mongo_connect,
    frameworks::web,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let client = mongo_connect().await;
    let (host, port) = get_address();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.clone()))
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
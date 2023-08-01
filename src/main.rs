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
    let client = mongo_connect().await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.clone()))
            .service(
                scope("/api/v1")
                    .service(web::user_web::add_user)
            )
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

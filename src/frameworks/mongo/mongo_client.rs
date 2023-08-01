use mongodb::Client;
use crate::frameworks::mongo::mongo_constants::DB_URI;

pub async fn mongo_connect() -> Client {
    let client = Client::with_uri_str(DB_URI)
        .await
        .expect("與資料庫連結失敗");

    client
}
use std::env;
use mongodb::Client;

pub async fn mongo_connect() -> Client {
    let db_uri = env::var("DB_URI").expect("DB_URI 環境變數不存在");

    let client = Client::with_uri_str(db_uri)
        .await
        .expect("與資料庫連結失敗");

    client
}
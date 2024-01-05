use std::env;

use mongodb::Client;

pub struct MongoClient {
    pub client: Client,
}

impl MongoClient {
    pub async fn connect() -> MongoClient {
        let db_uri = env::var("DB_URI").expect("請設定 DB_URI 環境變數");

        let client = Client::with_uri_str(db_uri)
            .await
            .expect("與資料庫連結失敗");

        MongoClient {
            client,
        }
    }
}
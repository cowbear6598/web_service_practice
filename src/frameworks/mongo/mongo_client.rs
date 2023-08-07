use std::env;
use mongodb::Client;
use shaku::Component;
use crate::adapters::mongo_trait::MongoClientTrait;

#[derive(Component)]
#[shaku(interface = MongoClientTrait)]
pub struct MongoClient {
    client: Client,
}

impl MongoClientTrait for MongoClient {}

pub async fn mongo_connect() -> MongoClientParameters {
    let db_uri = env::var("DB_URI").expect("請設定 DB_URI 環境變數");

    let client = Client::with_uri_str(db_uri)
        .await
        .expect("與資料庫連結失敗");

    MongoClientParameters {
        client,
    }
}
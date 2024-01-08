use std::env;

use anyhow::Result;
use async_trait::async_trait;
use mongodb::{Client, Collection};

use crate::{
    user::entities::user::User,
    user::repositories::user_repository::UserRepositoryTrait,
};

pub struct MongoUserRepository {
    pub coll: Collection<User>,
}

#[async_trait]
impl UserRepositoryTrait for MongoUserRepository {
    async fn register(&self, user: User) -> Result<()> {
        self.coll.insert_one(user, None).await?;

        Ok(())
    }
}

impl MongoUserRepository {
    pub fn new(client: &Client) -> Box<Self> {
        let db_name = env::var("DB_NAME").expect("請設定 DB_NAME 環境變數");
        let coll = client.database(&db_name).collection("user");

        Box::new(Self {
            coll,
        })
    }
}
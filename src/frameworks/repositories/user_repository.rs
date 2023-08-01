use async_trait::async_trait;
use mongodb::{Client, Collection};
use anyhow::{anyhow, Result};
use crate::{
    entities::user_entity::User,
    frameworks::mongo::mongo_constants::DB_NAME,
    adapters::user_trait::UserRepositoryTrait,
    frameworks::mongo::mongo_constants::USER_COLLECTION
};

pub struct UserRepository {
    pub collection: Collection<User>,
}

impl UserRepository {
    pub fn new(client: &Client) -> UserRepository {
        let collection = client.database(DB_NAME).collection(USER_COLLECTION);

        UserRepository {
            collection
        }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn add_user(&self, user: &User) -> Result<()> {
        self.collection.insert_one(user, None).await
            .map_err(|_| anyhow!("用戶已存在"))?;

        Ok(())
    }
}
use async_trait::async_trait;
use mongodb::{Client, Collection};
use anyhow::{Result};
use mongodb::bson::doc;
use crate::{
    entities::user_entity::User,
    frameworks::mongo::mongo_constants::DB_NAME,
    adapters::user_trait::UserRepositoryTrait,
    frameworks::mongo::mongo_constants::USER_COLLECTION,
};
use crate::frameworks::errors::user_error::UserError;

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
            .map_err(|_| UserError::AlreadyExists)?;

        Ok(())
    }

    async fn remove_user(&self, user_id: String) -> Result<()> {
        let filter = doc! {"user_id": user_id};

        self.collection.delete_one(filter, None).await
            .map_err(|_| UserError::NotExists)?;

        Ok(())
    }
}
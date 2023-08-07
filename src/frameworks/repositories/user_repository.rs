use async_trait::async_trait;
use mongodb::{Client, Collection};
use anyhow::{Result};
use mongodb::bson::doc;
use crate::{
    entities::user_entity::User,
    adapters::user_trait::UserRepositoryTrait,
    frameworks::errors::user_error::UserError,
};

pub struct UserRepository {
    pub collection: Collection<User>,
}

impl UserRepository {
    pub fn new(client: &Client) -> UserRepository {
        let collection = client.database("user").collection("user");

        UserRepository {
            collection
        }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn add_user(&self, user: User) -> Result<()> {
        self.collection.insert_one(user, None).await
            .map_err(|err| UserError::AddUserFail(err.to_string()))?;

        Ok(())
    }

    // async fn upload_avatar(&self, avatar_url: String) -> Result<()> {
    //     todo!()
    // }

    async fn remove_user(&self, user_id: String) -> Result<()> {
        let filter = doc! {"user_id": user_id};

        self.collection.delete_one(filter, None).await
            .map_err(|err| UserError::RemoveUserFail(err.to_string()))?;

        Ok(())
    }
}
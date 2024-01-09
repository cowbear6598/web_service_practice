use std::env;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use mongodb::{Client, Collection};
use mongodb::bson::doc;

use crate::{
    mongo::error::mongo_error::MongoError,
    user::entities::user::User,
    user::repositories::user_repository::UserRepositoryTrait,
    user::types::find_dto::FindDto,
};

pub struct MongoUserRepository {
    pub coll: Collection<User>,
}

#[async_trait]
impl UserRepositoryTrait for MongoUserRepository {
    async fn find(&self, dto: &FindDto) -> Result<Option<User>> {
        let mut filter = doc! {};

        if let Some(email) = &dto.email {
            filter.insert("email", email);
        }

        let user = self.coll.find_one(filter, None).await
            .map_err(|err| anyhow!(MongoError::Exception(err.to_string())))?;

        Ok(user)
    }

    async fn register(&self, user: User) -> Result<()> {
        self.coll.insert_one(user, None).await
            .map_err(|err| anyhow!(MongoError::Exception(err.to_string())))?;

        Ok(())
    }

    async fn remove(&self, email: String) -> Result<()> {
        let filter = doc! {
            "email": email
        };

        self.coll.delete_one(filter, None).await
            .map_err(|err| anyhow!(MongoError::Exception(err.to_string())))?;

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
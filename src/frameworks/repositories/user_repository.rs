use mongodb::{Client, Collection};
use crate::{
    entities::user::User,
    frameworks::mongo::mongo_constants::DB_NAME,
};

pub struct UserRepository {
    pub collection: Collection<User>,
}

impl UserRepository {
    pub fn new(client: &Client) -> UserRepository {
        let collection = client.database(DB_NAME).collection("users");

        UserRepository {
            collection
        }
    }
}
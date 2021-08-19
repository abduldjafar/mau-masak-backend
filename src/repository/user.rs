use mongodb::results::{ InsertOneResult};
use mongodb::{error::Error, Collection};

use crate::entity::users::Users;

#[derive(Clone)]
pub struct UsersRepository {
    collection: Collection,
}

impl UsersRepository{
    pub fn new(collection: Collection) -> UsersRepository {
        UsersRepository { collection }
    }

    pub async fn create(&self, data:&Users) -> Result<InsertOneResult, Error> {
        let serialized_movie = bson::to_bson(&data)?;
        let document = serialized_movie.as_document().unwrap();
        self.collection.insert_one(document.clone(), None).await
    }
}
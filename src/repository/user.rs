use mongodb::results::{ InsertOneResult};
use mongodb::{error::Error, Collection};
use mongodb::bson::{ doc};
use crate::entity::users::Users;
use bson::oid::ObjectId;
use bson::Document;

#[derive(Clone)]
pub struct UsersRepository {
    collection: Collection,
}

impl UsersRepository {
    pub fn new(collection: Collection) -> UsersRepository {
        UsersRepository { collection }
    }

    pub async fn create(&self, data: &Users) -> Result<InsertOneResult, Error> {
        let serialized_movie = bson::to_bson(&data)?;
        let document = serialized_movie.as_document().unwrap();
        self.collection.insert_one(document.clone(), None).await
    }

    pub async fn getById(&self, user_id: String) -> Result<Option<Document>, Error> {
        let id = ObjectId::with_string(user_id.as_str()).unwrap();
        self.collection.find_one(
            doc! {
                "_id": id
             },
            None,
        ).await
    }
}
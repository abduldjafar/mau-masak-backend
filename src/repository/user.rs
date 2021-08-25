use mongodb::results::{ InsertOneResult};
use mongodb::{error::Error, Collection,Cursor};
use mongodb::bson::{ doc};
use crate::entity::users::Users;
use bson::oid::ObjectId;
use bson::Document;
use bcrypt::{DEFAULT_COST, hash};


#[derive(Clone)]
pub struct UsersRepository {
    collection: Collection,
}

impl UsersRepository {
    pub fn new(collection: Collection) -> UsersRepository {
        UsersRepository { collection }
    }

    pub async fn create(&self, data: &mut Users) -> Result<InsertOneResult, Error> {
        let password_hashed = hash(&data.password, DEFAULT_COST);

        match password_hashed{
            Ok(password) => {
                data.password = password;
            }
            _ => {}
        }


        let serialized_movie = bson::to_bson(&data)?;
        let document = serialized_movie.as_document().unwrap();
        self.collection.insert_one(document.clone(), None).await
    }

    pub async fn get_by_id(&self, user_id: String) -> Result<Option<Document>, Error> {
        let id = ObjectId::with_string(user_id.as_str()).unwrap();
        self.collection.find_one(
            doc! {
                "_id": id
             },
            None,
        ).await
    }

    pub async fn get_by_email(&self, email: String) -> Result<Option<Document>, Error> {
        //let id = ObjectId::with_string(email.as_str()).unwrap();
        self.collection.find_one(
            doc! {
                "email": email
             },
            None,
        ).await
    }

    pub async fn get_all(&self) -> mongodb::error::Result<Cursor<Document>> {
        let cursor = self.collection.find(None, None).await;
        cursor
    }

    
}
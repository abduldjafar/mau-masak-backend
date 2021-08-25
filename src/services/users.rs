use crate::repository::user::UsersRepository;
use crate::entity::users::Users;
use mongodb::{error::Error,Cursor};
use mongodb::results::InsertOneResult;
use bson::Document;


pub struct UsersServiceManager {
    pub repository: UsersRepository,
}

// Api Servie Implementation
impl UsersServiceManager {
    pub fn new(repository: UsersRepository) -> Self {
        UsersServiceManager { repository }
    }

    pub async fn add(&self, data: &mut Users) -> Result<InsertOneResult, Error> {
        self.repository.create( data).await
    }

    pub async fn find_by_id(&self, id: String) -> Result<Option<Document>, Error> {
        self.repository.get_by_id(id).await
    }

    pub async fn find_by_email(&self, data: String) -> Result<Option<Document>, Error> {
        self.repository.get_by_email(data).await
    }

    pub async fn find_all(&self) -> mongodb::error::Result<Cursor<Document>> {
        let data = self.repository.get_all().await;
        data
    }
   
}

// Service Manager constructor
pub struct UserAppState {
    pub users_service_manager: UsersServiceManager,
}
use crate::repository::user::UsersRepository;
pub struct UsersServiceManager {
    pub repository: UsersRepository,
}

// Api Servie Implementation
impl UsersServiceManager {
    pub fn new(repository: UsersRepository) -> Self {
        UsersServiceManager { repository }
    }
}

// Service Manager constructor
pub struct UserAppState {
    pub users_service_manager: UsersServiceManager,
}
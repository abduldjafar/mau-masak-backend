use actix_web::{ web, HttpResponse, Responder};
use crate::entity::users::Users;
use crate::services::users::UserAppState;

pub async fn add_user(app_data: web::Data<UserAppState>, data: web::Json<Users>) -> impl Responder {

    let action_user =  app_data.users_service_manager.repository.create(&data).await;
    let result = web::block(move || action_user).await;

    match result {
        Ok(_) => HttpResponse::Ok().json("success"),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }

}
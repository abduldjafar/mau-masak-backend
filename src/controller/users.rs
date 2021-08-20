use actix_web::{ web,get, HttpResponse, Responder};
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

#[get("get/{user_id}")]
pub async fn getUser(app_data: web::Data<UserAppState>, web::Path((user_id)): web::Path<(String)>) -> impl Responder {

    let action_user =  app_data.users_service_manager.repository.getById(user_id).await;
    let result = web::block(move || action_user).await;


    match result {
        Ok(result) => HttpResponse::Ok().json(result.unwrap()),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }

}
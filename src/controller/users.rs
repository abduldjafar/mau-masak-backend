use actix_web::{ web,get, HttpResponse, Responder,Result};
use crate::entity::users::Users;
use crate::services::users::UserAppState;
use futures::{ TryStreamExt};
use bson::{Bson, Document};
use crate::entity::responses::ResponseBody;
use crate::constants;
use std::error::Error;

pub async fn add_user(app_data: web::Data<UserAppState>, mut data: web::Json<Users>) -> impl Responder {

    let action_user =  app_data.users_service_manager.add(&mut data).await;
    let result = web::block(move || action_user).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(ResponseBody::new(
            "success",
            200,
            false,
            constants::EMPTY,
        )),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().json(ResponseBody::new(
                "Error whe register user",
                500,
                true,
                constants::EMPTY,
            ))
        }
    }

}


pub async fn get_user_by_id(app_data: web::Data<UserAppState>, web::Path(user_id): web::Path<String>) -> Result<HttpResponse> {

    let action_user =  app_data.users_service_manager.find_by_id(user_id).await;
    let result = web::block(move || action_user).await;

    match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result.unwrap())),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            Ok(HttpResponse::InternalServerError().json(""))
        }
    }

}


pub async fn get_all(app_data: web::Data<UserAppState>) -> Result<HttpResponse>  {

    let action_user =  app_data.users_service_manager.find_all().await;
    let result = web::block(move || action_user).await;

    match result {
        Ok(mut result) => {
            let mut vec = Vec::new();
            while let Some(doc) = result.try_next().await.unwrap() {
                let user: Users = bson::from_bson(Bson::Document(doc)).unwrap();
                vec.push(user);
            }
            Ok(HttpResponse::Ok().json(ResponseBody::new(
                "success",
                200,
                false,
                vec,
            )))
        },
        Err(e) => {
            println!("Error while getting, {:?}", e);
            Ok(HttpResponse::InternalServerError().json(ResponseBody::new(
                "success",
                500,
                true,
                constants::EMPTY,
            )))
        }
    }
}

pub async fn login(app_data: web::Data<UserAppState>, mut data: web::Json<Users>) -> impl Responder {

    let action_user =  app_data.users_service_manager.find_by_email(data.email.to_string()).await;
    let result = web::block(move || action_user).await;
    match result {
        Ok(document) => {
            match document{
                Some(doc) => {
                    HttpResponse::Ok().json(ResponseBody::new(
                        "success",
                        200,
                        false,
                        crate::entity::jwt::UserToken::generate_token(data.email.to_string()),
                    ))
                }
                None => {
                    HttpResponse::Forbidden().json(ResponseBody::new(
                        "Failed!",
                        403,
                        false,
                        crate::constants::MESSAGE_USER_NOT_FOUND,
                    ))
                }
            }
        },
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().json(ResponseBody::new(
                "Error whe register user",
                500,
                true,
                constants::EMPTY,
            ))
        }
    }
}

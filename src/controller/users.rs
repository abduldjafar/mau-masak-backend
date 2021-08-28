use actix_web::{web, get, HttpResponse, Responder, Result, HttpRequest};
use crate::entity::users::Users;
use crate::services::users::UserAppState;
use futures::{ TryStreamExt};
use bson::{Bson, Document};
use crate::entity::responses::ResponseBody;
use crate::constants;
use std::error::Error;
use regex::Regex;
use bcrypt::{verify};
use std::ptr::null;
use std::borrow::Borrow;


pub async fn add_user(app_data: web::Data<UserAppState>, mut data: web::Json<Users>) -> impl Responder {
    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
    if !email_regex.is_match(data.email.as_str()){
        HttpResponse::BadRequest().json(ResponseBody::new(
            "Error whe register user",
            500,
            true,
            constants::MESSAGE_INTERNAL_SERVER_ERROR,
        ))
    }else {
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


}


pub async fn get_user_by_id(app_data: web::Data<UserAppState>,req: HttpRequest) -> Result<HttpResponse> {
    let user_id = extract_token(req);

    let action_user =  app_data.users_service_manager.find_by_id(user_id).await;
    let result = web::block(move || action_user).await;

    match result {
        Ok(results) =>{
            if results.is_none(){
                Ok(HttpResponse::InternalServerError().json("You don't have account"))
            }else{
                Ok(HttpResponse::Ok().json(results.unwrap()))
            }
        },
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
                let mut user: Users = bson::from_bson(Bson::Document(doc)).unwrap();
                user.password = "".to_string();

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
                "error when fetch data",
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
                    let user: Users = bson::from_bson(Bson::Document(doc)).unwrap();
                    let valid = verify(data.password.as_str(), user.password.as_str()).unwrap();
                    if valid{
                        HttpResponse::Ok().json(ResponseBody::new(
                            constants::MESSAGE_LOGIN_SUCCESS,
                            200,
                            false,
                            crate::entity::jwt::UserToken::generate_token(user.email, user.id.unwrap().to_hex()),
                        ))
                    }else{
                        HttpResponse::Unauthorized().json(ResponseBody::new(
                            constants::MESSAGE_LOGIN_FAILED,
                            401,
                            true,
                            "no data"
                        ))
                    }

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

pub  fn extract_token(request: HttpRequest) -> String{
    let req_headers:&str = request.headers().get("Authorization").unwrap().to_str().unwrap();
    let token = req_headers[6..req_headers.len()].trim();
    let decode_token = crate::entity::jwt::UserToken::decode_token(token.to_string());
    let result = decode_token.unwrap();
    result.claims.id

}

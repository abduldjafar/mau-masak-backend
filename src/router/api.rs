use actix_web::web;
use crate::controller::users;

pub fn user_api_auth(cfg: &mut web::ServiceConfig)  {
    //cfg.route("/save",web::post().to(users::add_user));
    cfg.service(
        web::resource("/get")
            .route(web::get().to(users::get_all))

    );
}

pub fn user_api(cfg: &mut web::ServiceConfig)  {
    //cfg.route("/save",web::post().to(users::add_user));
    cfg.service(
        web::scope("/v1")
            .route("/save",web::post().to(users::add_user))
            .route("/login",web::post().to(users::login))
    );
}
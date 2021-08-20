use actix_web::web;
use crate::controller::users;

pub fn userApi(cfg: &mut web::ServiceConfig)  {
    //cfg.route("/save",web::post().to(users::add_user));
    cfg.service(
        web::scope("/v1")
            .route("/save",web::post().to(users::add_user))
            .service(users::getUser)
    );
}
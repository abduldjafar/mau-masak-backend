use actix_web::web;
use crate::controller::users;

pub fn init(cfg: &mut web::ServiceConfig)  {
    cfg.route("/save",web::post().to(users::add_user));
}
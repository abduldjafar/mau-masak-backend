use actix_web::web;
use crate::controller::users;


pub fn user_api(cfg: &mut web::ServiceConfig)  {
    cfg.service(
        web::scope("/v1")
            .service(
                web::resource("/login")
                    .route(web::post().to(users::login))
            )
            .service(
                web::scope("/user")
                    .service(
                        web::resource("/add").route(web::post().to(users::add_user))
                    )
                    .service(
                        web::resource("/findall").route(web::get().to(users::get_all))
                            .wrap(crate::middlewares::auth::Authentication)
                    )
                    .service(
                        web::resource("/find").route(web::get().to(users::get_user_by_id))
                            .wrap(crate::middlewares::auth::Authentication)
                    )
            )
    );
}
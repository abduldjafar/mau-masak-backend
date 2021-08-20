use std::env;
use actix_cors::Cors;
use actix_web::{web,http, middleware, App, HttpServer,HttpRequest,Responder};
use dotenv::dotenv;

mod entity;
mod repository;
mod services;
mod controller;
mod router;
mod configuration;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    //setup collection name from env variable
    let user_collections = env::var("USER_COLLECTION").expect("DATABASE NAME is not in .env file");

    // set up collections
    let user_collection = configuration::mongo_config::init().await.collection(user_collections.as_str());

    // Gte the server URL
    let server_url = env::var("SERVER_URL").expect("SERVER URL is not in .env file");

    // Start the server
    HttpServer::new(move || {
        let user_repository = repository::user::UsersRepository::new(user_collection.clone());
        let users_service_manager = services::users::UsersServiceManager::new(user_repository);

        // cors
        let cors_middleware = Cors::default()
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);


        // Init http server
        App::new()
            .wrap(cors_middleware)
            .wrap(middleware::Logger::default())
            .data(services::users::UserAppState{users_service_manager})
            .configure(router::api::userApi)
            .route("/", web::get().to(greet))
    })
        .bind(server_url)?
        .run()
        .await
}
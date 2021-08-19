use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client, Database};
use std::env;

pub async fn init() -> Database{
    dotenv().ok();

    // init logger middleware
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // Parse a connection string into an options struct.
    let database_url = env::var("DATABASE_URL").expect("DATABASE URL is not in .env file");
    let client_options = ClientOptions::parse(&database_url).await.unwrap();

    // Get the reference to Mongo DB
    let client = Client::with_options(client_options).unwrap();

    // get the reference to the Data Base
    let database_name = env::var("DATABASE_NAME").expect("DATABASE NAME is not in .env file");
    let db = client.database(&database_name);

    db
}
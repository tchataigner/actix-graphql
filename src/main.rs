extern crate core;

use crate::db::get_db_client;
use crate::handlers::register;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};

mod db;
mod handlers;
mod schemas;

pub struct AppState {
    mongo_client: mongodb::Client,
    mongo_database: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let client = get_db_client().await;
    let mongo_database = std::env::var("MONGODB_DATABASE").expect("MONGODB_DATABASE must be set");

    log::info!("starting HTTP server on port 8080");
    log::info!("GraphiQL playground: http://localhost:8080/graphiql");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                mongo_client: client.clone(),
                mongo_database: mongo_database.clone(),
            }))
            .configure(register)
            .wrap(Cors::permissive())
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

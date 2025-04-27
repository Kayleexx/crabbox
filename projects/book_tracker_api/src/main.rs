mod handlers;  
mod db;
mod models;
mod schema;

use actix_web::{App, HttpServer, web};
use crate::handlers as other_handlers;  
use crate::db as other_db;     
use crate::db::DbPool;
use dotenvy::{dotenv, from_path};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    match from_path("/home/minerva/crabbox/projects/book_tracker_api/.env") {
        Ok(_) => println!(".env file loaded successfully."),
        Err(e) => panic!("Failed to load .env file: {}", e),
    }


    match env::var("DATABASE_URL") {
        Ok(db_url) => {
            println!("DATABASE_URL loaded: {}", db_url);
        }
        Err(_) => panic!("DATABASE_URL must be set!"),
    }

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = diesel::r2d2::ConnectionManager::<diesel::PgConnection>::new(database_url);
    let pool = diesel::r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/books").route(web::get().to(other_handlers::get_books)))
            .service(web::resource("/books").route(web::post().to(other_handlers::create_book)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
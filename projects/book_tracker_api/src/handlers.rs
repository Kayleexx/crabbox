use actix_web::{web, HttpResponse, Responder};
use crate::models::Book;
use crate::schema::books;
use crate::db::DbPool;
use diesel::prelude::*;

pub async fn create_book(
    new_book: web::Json<Book>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    let new_book = new_book.into_inner(); // Deserialize JSON into Book

    // Insert into database
    let result = diesel::insert_into(books::table)
        .values(&new_book)
        .get_result::<Book>(&mut conn);

    match result {
        Ok(book) => HttpResponse::Created().json(book), // Return the created book
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()), // Return error message
    }
}

pub async fn get_books(pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    
    // Fetch all books from the database
    let result = crate::schema::books::table
        .load::<Book>(&mut conn);

    match result {
        Ok(books) => HttpResponse::Ok().json(books),  // Return the list of books as JSON
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),  // Handle errors
    }
}

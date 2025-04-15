use actix_web::{get, post, delete, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use std::collections::HashMap;
mod models;
use models::{Note, CreateNote};

type Notes = Mutex<HashMap<u32, Note>>;  

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Notes API!\n")
}

#[post("/notes")]
async fn create_note(
    data: web::Data<Notes>,
    note: web::Json<CreateNote>,
) -> impl Responder {
    let mut notes = data.lock().unwrap();
    let new_id = notes.len() as u32 + 1; 
    let new_note = Note {
        id: new_id,
        title: note.title.clone(),
        content: note.content.clone(),
    };
    notes.insert(new_id, new_note.clone());  
    HttpResponse::Created().json(new_note)
}

#[get("/notes")]
async fn get_notes(data: web::Data<Notes>) -> impl Responder {
    let notes = data.lock().unwrap();
    let notes_vec: Vec<_> = notes.values().cloned().collect();  
    HttpResponse::Ok().json(notes_vec)
}

#[get("/notes/{id}")]
async fn get_note_by_id(
    data: web::Data<Notes>,
    path: web::Path<u32>,
) -> impl Responder {
    let notes = data.lock().unwrap();
    let note_id = path.into_inner();

    if let Some(note) = notes.get(&note_id) {
        HttpResponse::Ok().json(note)
    } else {
        HttpResponse::NotFound().body(format!("Note with id {} not found", note_id))
    }
}

#[delete("/notes/{id}")]
async fn delete_note(
    data: web::Data<Notes>,
    path: web::Path<u32>,
) -> impl Responder {
    let mut notes = data.lock().unwrap();
    let note_id = path.into_inner();

    if notes.remove(&note_id).is_some() {
        HttpResponse::Ok().body(format!("Note with id {} deleted", note_id))
    } else {
        HttpResponse::NotFound().body(format!("Note with id {} not found", note_id))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");

    let notes = web::Data::new(Mutex::new(HashMap::<u32, Note>::new())); 

    HttpServer::new(move || {
        App::new()
            .app_data(notes.clone()) 
            .service(index)
            .service(create_note)
            .service(get_notes)
            .service(get_note_by_id)
            .service(delete_note)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

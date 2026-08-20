use std::env;

use actix_web::{web, App, HttpResponse, HttpServer, Result};
use dotenvy::dotenv;

use notes_core::config;
use notes_core::db::Database;

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("Invalid port");

    let mut db_path = config::get_db_path();
    
    db_path.pop();
    db_path.pop();
    db_path.pop();
    db_path.push("notes.db");

    let db = Database::open(&db_path)
        .await
        .expect("Failed to open database");

    let db = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .route("/", web::get().to(index))
            .route("/notes", web::get().to(handlers::notes::get_all_notes))
            .route("/note", web::get().to(handlers::notes::get_note))
            .route("/notes", web::post().to(handlers::notes::create_note))
            .route("/notes", web::patch().to(handlers::notes::update_note))
            .route("/notes", web::delete().to(handlers::notes::delete_note))
            .route("/notes/all", web::delete().to(handlers::notes::delete_all_notes))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Notes API"))
}

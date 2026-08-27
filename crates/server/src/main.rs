use std::env;

use actix_web::{web, App, HttpResponse, HttpServer, Result, middleware::from_fn};
use dotenvy::dotenv;

use notes_core::config;
use notes_core::db::Database;

use crate::auth::middleware::auth_middleware;

mod handlers;
mod auth;

pub struct AppState {
    pub db: Database,
    pub jwt: JwtService,
}

pub struct JwtService {
    pub(crate) secret: String,
}

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

    let jwt = JwtService { 
        secret: env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY is not configured")
    };

    let app_state = web::Data::new(AppState {
        db,
        jwt,
    });

    // Actix routes
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(index))
            // Auth 
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(handlers::auth::login))
                    .route("/register", web::post().to(handlers::auth::register))
            )
            .service(
                web::scope("/api")
                    .wrap(from_fn(auth_middleware))

                    // Notes
                    .route("/notes/all", web::get().to(handlers::notes::get_all_notes))
                    .route("/notes", web::get().to(handlers::notes::get_note))
                    .route("/notes", web::post().to(handlers::notes::create_note))
                    .route("/notes", web::patch().to(handlers::notes::update_note))
                    .route("/notes", web::delete().to(handlers::notes::delete_note))
                    .route("/notes/all", web::delete().to(handlers::notes::delete_all_notes))
                    
                    // Tags
                    .route("/tags/all", web::get().to(handlers::tags::get_all_tags))
                    .route("/tags", web::get().to(handlers::tags::get_tags_for_note))
                    .route("/tags", web::post().to(handlers::tags::create_tag))
                    .route("/tags", web::delete().to(handlers::tags::delete_tag))

                    // Search
                    .route("/search/tags", web::get().to(handlers::search::search_tags))
                    .route("/search/notes", web::get().to(handlers::search::search_notes))

                    // Users
                    .route("/users", web::delete().to(handlers::users::delete_user))
                )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Notes API"))
}

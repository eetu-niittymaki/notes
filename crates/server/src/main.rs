use actix_web::{get, App, HttpServer, HttpResponse, Responder, web};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Sammy Todo API!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(get_todo)
            .service(create_todo)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

use serde::{Serialize, Deserialize}; // Added Deserialize to imports

#[derive(Serialize, Deserialize)] // Ensure Deserialize is imported
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

#[get("/todos")]
async fn create_todo(todo: web::Json<Todo>) -> impl Responder {
    HttpResponse::Ok().body(serde_json::to_string(&todo).unwrap())
}

#[get("/todos/{id}")]
async fn get_todo(id: web::Path<i32>) -> impl Responder {
    let todo = Todo {
        id: id.into_inner(),
        title: "Learn Actix-Web".to_string(),
        completed: false,
    };
    HttpResponse::Ok().body(serde_json::to_string(&todo).unwrap())
}
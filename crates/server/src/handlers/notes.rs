use actix_web::{web, HttpResponse, Result};

use notes_core::db::Database;

use notes_core::models::note::CreateNote;

pub async fn get_notes(
    db: web::Data<Database>,
) -> Result<HttpResponse, actix_web::Error> {
    let notes = db
        .notes()
        .get_all_with_tags()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(notes))
}

pub async fn create_note(
    db: web::Data<Database>,
    payload: web::Json<CreateNote>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = db
        .notes()
        .create(payload.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(id))
}
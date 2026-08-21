use actix_web::{web, HttpResponse, Result};

use notes_core::db::Database;
use notes_core::models::note::NoteSearchQuery;
use notes_core::models::tag::TagSearchQuery;

pub async fn search_tags(
    db: web::Data<Database>,
    query: web::Query<TagSearchQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let tags = db
        .search()
        .tags(query.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(tags))
}

pub async fn search_notes(
    db: web::Data<Database>,
    query: web::Query<NoteSearchQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let notes = db
        .search()
        .notes(query.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(notes))
}

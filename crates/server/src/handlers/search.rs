use actix_web::{web, HttpResponse, Result};

use notes_core::models::note::NoteSearchQuery;
use notes_core::models::tag::TagSearchQuery;

use crate::auth::user::AuthenticatedUser;
use crate::AppState;

pub async fn search_tags(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Query<TagSearchQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let tags = state
        .db
        .search()
        .tags(user.id, query.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(tags))
}

pub async fn search_notes(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Query<NoteSearchQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let notes = state
        .db
        .search()
        .notes(user.id, query.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(notes))
}

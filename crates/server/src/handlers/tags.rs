use actix_web::{web, HttpResponse, Result};

use notes_core::models::tag::{
    TagQuery,
    CreateTagQuery,
    DeleteTagQuery
};

use crate::auth::user::AuthenticatedUser;
use crate::AppState;

pub async fn get_all_tags(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, actix_web::Error> {
    let tags = state
        .db
        .tags()
        .all(user.id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(tags))
}

pub async fn get_tags_for_note(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Query<TagQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let tags = state
        .db
        .tags()
        .for_note(user.id, query.note_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(tags))
}

pub async fn create_tag(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Query<CreateTagQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let tag = state
        .db
        .tags()
        .add(user.id, query.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(tag))
}

pub async fn delete_tag(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Query<DeleteTagQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let deleted = state
        .db
        .tags()
        .delete(user.id, query.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if deleted == 0 {
        return Err(actix_web::error::ErrorNotFound(
            "Tag not found",
        ));
    }

    Ok(HttpResponse::NoContent().finish())
}
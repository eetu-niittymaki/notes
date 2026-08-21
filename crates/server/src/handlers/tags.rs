use actix_web::{web, HttpResponse, Result};

use notes_core::db::Database;
use notes_core::models::tag::{
    TagQuery,
    CreateTagQuery,
    DeleteTagQuery
};

pub async fn get_all_tags(
    db: web::Data<Database>,
) -> Result<HttpResponse, actix_web::Error> {
    let tags = db
        .tags()
        .all()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(tags))
}

pub async fn get_tags_for_note(
    db: web::Data<Database>,
    query: web::Query<TagQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let tags = db
        .tags()
        .for_note(query.note_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(tags))
}

pub async fn create_tag(
    db: web::Data<Database>,
    query: web::Query<CreateTagQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let tag = db
        .tags()
        .add(query.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(tag))
}

pub async fn delete_tag(
    db: web::Data<Database>,
    query: web::Query<DeleteTagQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let delete = db
        .tags()
        .delete(query.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(delete))
}
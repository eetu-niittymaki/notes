use actix_web::{web, HttpResponse, Result};

use notes_core::db::Database;
use notes_core::models::note::{
    CreateNoteQuery,
    DeleteNoteQuery, 
    NoteQuery, 
    GetAllNotesQuery,
    NoteUpdate, 
    UpdateNoteQuery
};

pub async fn get_note(
    db: web::Data<Database>,
    query: web::Query<NoteQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let notes = db
        .notes()
        .get(query.id, query.user_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(notes))
}

pub async fn get_all_notes(
    db: web::Data<Database>,
    query: web::Query<GetAllNotesQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let notes = db
        .notes()
        .get_all_with_tags(query.user_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(notes))
}

pub async fn create_note(
    db: web::Data<Database>,
    query: web::Json<CreateNoteQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = db
        .notes()
        .create(query.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(id))
}

pub async fn update_note(
    db: web::Data<Database>,
    query: web::Query<UpdateNoteQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let update = match (&query.title, &query.content) {
        (Some(title), None) => NoteUpdate::Title(title.clone()),

        (None, Some(content)) => NoteUpdate::Content(content.clone()),

        (Some(_), Some(_)) => {
            return Err(actix_web::error::ErrorBadRequest(
                "Provide either title or content, not both",
            ));
        }

        (None, None) => {
            return Err(actix_web::error::ErrorBadRequest(
                "Provide either title or content",
            ));
        }
    };

    let updated = db
        .notes()
        .update(query.id, query.user_id, update)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if updated == 0 {
        return Err(actix_web::error::ErrorNotFound("Note not found"));
    }

    Ok(HttpResponse::NoContent().finish())
}


pub async fn delete_note(
    db: web::Data<Database>,
    query: web::Query<DeleteNoteQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let deleted = db.notes()
                .delete(query.id)
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;

    if deleted == 0 {
        return Err(actix_web::error::ErrorNotFound(
            "Note not found",
        ));
    }

    Ok(HttpResponse::NoContent().finish())
}

pub async fn delete_all_notes(
    db: web::Data<Database>,
) -> Result<HttpResponse, actix_web::Error> {
    let deleted = db
        .notes()
        .delete_all()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "deleted": deleted
    })))
}
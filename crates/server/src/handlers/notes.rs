use actix_web::{web, HttpResponse, Result};

use notes_core::models::note::{
    CreateNoteQuery,
    DeleteNoteQuery, 
    NoteQuery, 
    NoteUpdate, 
    UpdateNoteQuery
};

use crate::auth::user::AuthenticatedUser;
use crate::AppState;

pub async fn get_note(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Query<NoteQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let res = state
        .db
        .notes()
        .get(user.id, query.id)
        .await;
        //.map_err(actix_web::error::ErrorInternalServerError)?;

    let note = res
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(note))
}

pub async fn get_all_notes(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, actix_web::Error> {
    let notes = state
        .db
        .notes()
        .get_all_with_tags(user.id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(notes))
}

pub async fn create_note(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Json<CreateNoteQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = state
        .db
        .notes()
        .create(user.id, query.into_inner())
        .await
        .map_err(|e| {
            eprintln!("create note failed: {:?}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;

    Ok(HttpResponse::Created().json(id))
}

pub async fn update_note(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
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

    let updated = state
        .db
        .notes()
        .update(user.id, query.id, update)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if updated == 0 {
        return Err(actix_web::error::ErrorNotFound("Note not found"));
    }

    Ok(HttpResponse::NoContent().finish())
}


pub async fn delete_note(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Query<DeleteNoteQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let deleted = state
                .db
                .notes()
                .delete(user.id, query.id)
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
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, actix_web::Error> {
    let deleted = state
        .db
        .notes()
        .delete_all(user.id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "deleted": deleted
    })))
}
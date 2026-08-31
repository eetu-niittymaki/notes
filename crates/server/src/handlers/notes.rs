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
use notes_core::error::Error;

async fn add_history(
    state: &web::Data<AppState>,
    user_id: i64,
    note_id: i64,
    version_number: i64,
    title: &str,
    content: &str
) -> Result<(), actix_web::Error> {
    state
        .db
        .history()
        .create(
            user_id,
            note_id,
            version_number,
            title,
            content,
        )
        .await
        .map_err(|e| {
            eprintln!("create note history failed: {:?}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;

    Ok(())
}

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

    let note = res.map_err(|err| match err {
        Error::NotFound => {
            actix_web::error::ErrorNotFound("Note not found")
        }
        err => actix_web::error::ErrorInternalServerError(err),
    })?;

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
        .create(user.id, query.clone())
        .await
        .map_err(|e| {
            eprintln!("create note failed: {:?}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;

    add_history(
        &state, 
        user.id, 
        id, 
        1, 
        &query.title, 
        &query.content
    ).await?;

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

    // Get the latest edits version number 
    let latest_version = state
        .db
        .history()
        .newest(user.id, query.id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // Get that notes current title and content
    let note = state
        .db
        .notes()
        .get(user.id, query.id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    add_history(
        &state, 
        user.id, 
        query.id, 
        latest_version.version_number + 1, 
        &note.title, 
        &note.content
    ).await?;

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
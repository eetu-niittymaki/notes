use actix_web::{web, HttpResponse, Result};

use notes_core::models::history::{
    GetHistoryQuery, 
    GetVersionQuery, 
    NoteHistory, 
    RestoreNoteQuery
};

use crate::auth::user::AuthenticatedUser;
use crate::AppState;
use notes_core::error::Error;

pub async fn add_history(
    state: &web::Data<AppState>,
    user_id: i64,
    note_id: i64,
    version_number: i64,
    operation: &str,
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
            operation,
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

pub async fn get_full_history(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Query<GetHistoryQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let res = state
        .db
        .history()
        .get_all(user.id, query.note_id)
        .await;

    let history = res.map_err(|err| match err {
        Error::NotFound => {
            actix_web::error::ErrorNotFound("Note history not found")
        }
        err => actix_web::error::ErrorInternalServerError(err),
    })?;

    Ok(HttpResponse::Ok().json(history))
}

pub async fn get_version(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Query<GetVersionQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let res = state
        .db
        .history()
        .get_one(user.id, query.note_id, query.version_number)
        .await;

    let history = res.map_err(|err| match err {
        Error::NotFound => {
            actix_web::error::ErrorNotFound("Note history not found")
        }
        err => actix_web::error::ErrorInternalServerError(err),
    })?;

    Ok(HttpResponse::Ok().json(history))
}

pub async fn get_latest_version(
    state: &web::Data<AppState>,
    user_id: i64,
    note_id: i64,
) -> Result<NoteHistory>{
    Ok(state
        .db
        .history()
        .newest(user_id, note_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?)
}

pub async fn restore_version(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Query<RestoreNoteQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let res = state
        .db
        .history()
        .restore(user.id, query.note_id, &query.title, &query.content)
        .await;

    let history = res.map_err(|err| match err {
        Error::NotFound => {
            actix_web::error::ErrorNotFound("Note history not found")
        }
        err => actix_web::error::ErrorInternalServerError(err),
    })?;

    let latest_version = get_latest_version(&state, user.id, query.note_id).await?;

    add_history(
        &state, 
        user.id, 
        query.note_id, 
        latest_version.version_number + 1, 
        "Restored",
        &query.title, 
        &query.content
    ).await?;

    Ok(HttpResponse::Ok().json(history))
}

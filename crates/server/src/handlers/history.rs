use actix_web::{web, HttpResponse, Result};

use notes_core::models::history::{
    GetHistoryQuery,
    GetVersionQuery,
    RestoreNoteQuery
};

use crate::auth::user::AuthenticatedUser;
use crate::AppState;
use notes_core::error::Error;

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

    Ok(HttpResponse::Ok().json(history))
}
use actix_web::{web, HttpResponse, Result};

use notes_core::db::Database;

use notes_core::models::note::{CreateNote, NoteQuery, NoteSelector};

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

pub async fn delete_note(
    query: web::Query<NoteQuery>,
    db: web::Data<Database>,
) -> Result<HttpResponse, actix_web::Error> {
    match (&query.id, &query.title) {
        (Some(id), None) => {
            db.notes()
                .delete(NoteSelector::Id(*id))
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;
        }

        (None, Some(title)) => {
            db.notes()
                .delete(NoteSelector::Title(title))
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;
        }

        (Some(_), Some(_)) => {
            return Err(actix_web::error::ErrorBadRequest(
                "Provide either id or title, not both",
            ));
        }

        (None, None) => {
            return Err(actix_web::error::ErrorBadRequest(
                "Provide either id or title",
            ));
        }

        _ => {
            return Err(actix_web::error::ErrorBadRequest(
                "Provide either id or title",
            ));
        }
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
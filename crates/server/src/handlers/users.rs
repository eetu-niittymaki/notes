use actix_web::{web, HttpResponse, Result};

use notes_core::db::Database;
use notes_core::models::user::{
    //User,
    //NewUser,
    //GetUser,
    DeleteUser,
};

// Not used
/* 
pub async fn create_user(
    db: web::Data<Database>,
    query: web::Query<NewUser>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = db
        .users()
        .create(query.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(User::from(user)))
}

// Not used
pub async fn get_user(
    db: web::Data<Database>,
    query: web::Query<GetUser>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = db
        .users()
        .get_by_id(query.user_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(User::from(user.unwrap())))
}
*/

pub async fn delete_user(
    db: web::Data<Database>,
    query: web::Query<DeleteUser>,
) -> Result<HttpResponse, actix_web::Error> {
    db
        .users()
        .delete(query.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}


use actix_web::{web, HttpResponse, Result};

use notes_core::db::Database;
use notes_core::models::user::{
    User,
    NewUser,
    DeleteUser,
    LoginUser
};

pub async fn add_user(
    db: web::Data<Database>,
    query: web::Query<NewUser<'_>>,
) {
    todo!()
}

pub async fn get_user(
    db: web::Data<Database>,
    query: web::Query<LoginUser>,
) {
    todo!()
}

pub async fn delete_user(
    db: web::Data<Database>,
    query: web::Query<DeleteUser>,
) {
    todo!()
}

pub async fn authenticate_user(
    db: web::Data<Database>,
    query: web::Query<LoginUser>,
) {
    todo!()
}
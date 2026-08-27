use actix_web::{web, HttpResponse, Result};

use notes_core::models::auth::{
    LoginRequest,
    RegisterRequest
};

use crate::{
    auth::tokens::create_tokens,
    AppState,
};

pub async fn login(
    state: web::Data<AppState>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = state
        .db
        .auth()
        .login(req.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid credentials"))?;

    let tokens = create_tokens(&user, &state.jwt)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(tokens))
}

pub async fn register(
    state: web::Data<AppState>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = state
        .db
        .auth()
        .register(req.into_inner())
        .await
        .map_err(|err| match err {
            notes_core::error::Error::UserAlreadyExists =>
                actix_web::error::ErrorConflict("Username already taken"),
            _ =>
                actix_web::error::ErrorInternalServerError(err),
        })?;

    let tokens = create_tokens(&user, &state.jwt)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(tokens))
}

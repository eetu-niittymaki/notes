use actix_web::{
    HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    middleware::Next,
    web, Error,
};

use crate::{AppState, auth::tokens::verify_access_token};

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| ErrorUnauthorized("Missing authorization"))?;

    let header = header
        .to_str()
        .map_err(|_| ErrorUnauthorized("Invalid authorization"))?;

    let token = header
        .strip_prefix("Bearer ")
        .ok_or_else(|| ErrorUnauthorized("Invalid authorization"))?;

    let state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ErrorUnauthorized("Application state unavailable"))?;

    let claims = verify_access_token(token, &state.jwt)
        .map_err(|_| ErrorUnauthorized("Invalid authorization"))?;

    // Make claims available to the handler
    req.extensions_mut().insert(claims);

    next.call(req).await
}
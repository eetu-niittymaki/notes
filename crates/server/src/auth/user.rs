use actix_web::{
    dev::Payload,
    error::ErrorUnauthorized,
    Error,
    FromRequest,
    HttpRequest,
    HttpMessage,
};
use futures_util::future::{ready, Ready};

use super::tokens::Claims;

pub struct AuthenticatedUser {
    pub id: i64,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &HttpRequest,
        _: &mut Payload,
    ) -> Self::Future {
        let claims = req
            .extensions()
            .get::<Claims>()
            .cloned();

        match claims {
            Some(claims) => ready(Ok(AuthenticatedUser {
                id: claims.sub,
            })),

            None => ready(Err(
                ErrorUnauthorized("Not authenticated")
            )),
        }
    }
}
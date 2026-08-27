use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chrono::Utc;
use jsonwebtoken::{encode, decode, Algorithm, EncodingKey, DecodingKey, Validation, Header};
use serde::{Deserialize, Serialize};

use notes_core::error::Result;
use notes_core::models::user::User;

use crate::JwtService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub iss: String,
    pub aud: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tokens {
    pub access_token: String,
    //pub refresh_token: String,
}


pub fn create_tokens(user: &User, jwt: &JwtService) -> Result<Tokens> {
    let access_token = create_access_token(user, jwt)?;

    //let refresh_token = create_refresh_token();

    Ok(Tokens {
        access_token,
        //refresh_token,78
    })
}

fn create_access_token(user: &User, jwt: &JwtService) -> Result<String> {
    let now = Utc::now().timestamp() as usize;

    let claims = Claims {
        sub: user.id,
        iss: "notes-app".to_owned(),
        aud: "notes-app".to_owned(),
        iat: now,
        exp: now + 30 * 60,
    };

    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(jwt.secret.as_bytes()),
    )?;

    Ok(token)
}

fn create_refresh_token() -> String {
    let bytes: [u8; 32] = rand::random();

    URL_SAFE_NO_PAD.encode(bytes)
}

pub fn verify_access_token(token: &str, jwt: &JwtService) -> Result<Claims> {
    let mut validation = Validation::new(Algorithm::HS256);

    validation.set_issuer(&["notes-app"]);
    validation.set_audience(&["notes-app"]);

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt.secret.as_bytes()),
        &validation,
    )?;

    Ok(token_data.claims)
}
use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::future::{ready, Ready};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::structs::{AuthedUser, Claims};

// TODO: move this to an environment variable before this ever goes near production.
const JWT_SECRET: &[u8] = b"kartikey_WEB3_CEX_SECRET_KEY";

pub fn create_token(username: String) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + 60 * 60 * 24; // 24 hours from now

    let claims = Claims {
        sub: username,
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET)).unwrap()
}

pub fn decode_token(token: &str) -> Option<String> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )
    .map(|data| data.claims.sub)
    .ok()
}

impl FromRequest for AuthedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|token| token.to_str().ok());
        let token = match auth_header {
            Some(token) if token.starts_with("Bearer ") => &token[7..],
            _ => {
                return ready(Err(actix_web::error::ErrorUnauthorized(
                    "Missing or malformed Authorization header",
                )))
            }
        };

        match decode_token(token) {
            Some(username) => ready(Ok(AuthedUser { username })),
            None => ready(Err(actix_web::error::ErrorUnauthorized(
                "Invalid or expired token",
            ))),
        }
    }
}

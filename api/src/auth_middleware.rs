//! JWT-based authentication middleware for Poem.
//!
//! Extracts the bearer token from the `Authorization` header and verifies it,
//! exposing the user ID to downstream handlers via the [`UserId`] extractor.

use std::env;

use jsonwebtoken::{DecodingKey, Validation, decode};
use poem::{Error, FromRequest, Request, RequestBody, Result, http::StatusCode};

use crate::routes::user::Claims;

/// Poem request extractor that resolves a [`UserId`] from a JWT token.
pub struct UserId(pub String);

impl<'a> FromRequest<'a> for UserId {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        let token = req
            .headers()
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| Error::from_string("Missing token", StatusCode::UNAUTHORIZED))?;

        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| Error::from_string("token malformed", StatusCode::UNAUTHORIZED))?;

        Ok(UserId(token_data.claims.sub))
    }
}


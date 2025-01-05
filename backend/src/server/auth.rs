use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::cognito;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    pub sub: String,
    pub exp: usize,
}

fn extract_token(req: &Request) -> Result<&str, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(&auth_header[7..])
}

pub async fn verify_jwt(
    State(jwks): State<Arc<cognito::CognitoJWKS>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = extract_token(&req)?;

    let header = decode_header(token).map_err(|_| StatusCode::BAD_REQUEST)?;

    let kid = header.kid.ok_or(StatusCode::BAD_REQUEST)?;

    let jwk = jwks
        .keys
        .iter()
        .find(|k| k.kid == kid)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&["7lkjn4ni4rpv1a6co5pgt20p79"]);
    validation.set_required_spec_claims(&[
        "sub",
        "iss",
        "client_id",
        "origin_jti",
        "event_id",
        "token_use",
        "scope",
        "auth_time",
        "exp",
        "iat",
        "username",
    ]);

    let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let token_data = decode::<JWTClaims>(token, &decoding_key, &validation)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Store claims in request extensions for handlers to access.
    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}

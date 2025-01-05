use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
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
    let validation = Validation::new(Algorithm::RS256);

    for jwk in &jwks.keys {
        let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if let Ok(token_data) = decode::<JWTClaims>(token, &decoding_key, &validation) {
            req.extensions_mut().insert(token_data.claims);
            return Ok(next.run(req).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

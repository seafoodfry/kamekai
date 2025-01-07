use anyhow::{Context, Result};
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tracing::{info, instrument, warn};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonWebKeySet {
    pub keys: Vec<Jwk>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Jwk {
    pub kid: String,
    pub kty: String,
    pub alg: String,
    pub n: String,
    pub e: String,
    pub r#use: String,
}

#[derive(Debug)]
pub struct JwkManager {
    jwks: Arc<RwLock<CachedJwks>>,
    cognito_client_id: String,
    jwks_url: String,
}

#[derive(Clone, Debug)]
struct CachedJwks {
    jwks: JsonWebKeySet,
    last_refresh: SystemTime,
}

impl JwkManager {
    pub async fn new(cognito_user_pool: String, cognito_client_id: String) -> Result<Self> {
        let jwks_url = format!("https://{}/.well-known/jwks.json", cognito_user_pool);

        // Create instance first with empty/initial state.
        let manager = Self {
            jwks: Arc::new(RwLock::new(CachedJwks {
                jwks: JsonWebKeySet { keys: vec![] }, // Empty initial state.
                last_refresh: SystemTime::now(),
            })),
            cognito_client_id,
            jwks_url,
        };

        // Then fetch initial JWKs using the instance method
        let jwks = manager
            .fetch_jwks()
            .await
            .context("Failed to fetch initial JWKs")?;

        // Update the cache.
        {
            let mut cached = manager.jwks.write().await;
            cached.jwks = jwks;
        } // Write lock is dropped here.

        Ok(manager)
    }

    async fn fetch_jwks(&self) -> Result<JsonWebKeySet> {
        let client = Client::new();
        let jwks: JsonWebKeySet = client
            .get(&self.jwks_url)
            .send()
            .await
            .with_context(|| {
                format!(
                    "Error obtaining JWKs for the OIDC authority {}",
                    self.jwks_url
                )
            })?
            .json()
            .await
            .with_context(|| {
                format!(
                    "Error parsing JWKs for the OIDC authority {}",
                    self.jwks_url
                )
            })?;
        tracing::info!("Fetched JWKs: {:?}", jwks);
        Ok(jwks)
    }

    pub async fn get_jwks(&self) -> Result<JsonWebKeySet> {
        let cached = self.jwks.read().await;

        // Refresh if JWKs are older than 5 minutes.
        if cached.last_refresh.elapsed()? > Duration::from_secs(300) {
            // Drop read lock before acquiring write lock to prevent deadlock
            drop(cached);

            // Acquire write lock
            let mut write_guard = self.jwks.write().await;

            // Double-check after acquiring write lock because someone else may have
            // refreshed the cache while we were waiting for the write lock.
            if write_guard.last_refresh.elapsed()? > Duration::from_secs(3600) {
                let new_jwks = self.fetch_jwks().await?;
                write_guard.jwks = new_jwks;
                write_guard.last_refresh = SystemTime::now();
            }

            // Clone the jwks before dropping the write lock.
            let jwks = write_guard.jwks.clone();
            return Ok(jwks);
        }

        Ok(cached.jwks.clone())
    }

    pub fn get_client_id(&self) -> &str {
        &self.cognito_client_id
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CognitoClaims {
    pub sub: String, // Making it public because we log the user ID elsewhere.
    iss: String,
    client_id: String,
    origin_jti: String,
    token_use: String,
    scope: String,
    auth_time: i64,
    exp: i64,
    iat: i64,
    jti: String,
    email: String,
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

#[instrument(
    name = "verify_jwt", 
    fields(client_id = %jwk_manager.get_client_id()),
    skip(jwk_manager, req, next),  // Skip complex types
    err
)]
pub async fn verify_jwt(
    State(jwk_manager): State<Arc<JwkManager>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = match extract_token(&req) {
        Ok(t) => {
            info!("token extracted successfully");
            t
        }
        Err(status) => {
            warn!("token extraction failed");
            return Err(status);
        }
    };

    let jwks = jwk_manager
        .get_jwks()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let header = match decode_header(token) {
        Ok(h) => {
            info!("decoded JWT header");
            h
        }
        Err(_) => {
            warn!("failed to decode JWT header");
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let kid = header.kid.ok_or_else(|| {
        warn!("no kid in JWT header");
        StatusCode::BAD_REQUEST
    })?;

    let jwk = match jwks.keys.iter().find(|k| k.kid == kid) {
        Some(k) => {
            info!(kid = %kid, "found matching JWK");
            k
        }
        None => {
            warn!(kid = %kid, "no matching JWK found");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let validation = Validation::new(Algorithm::RS256);
    // validation.set_required_spec_claims(&[
    //     "sub",
    //     "iss",
    //     "client_id",
    //     "origin_jti",
    //     "event_id",
    //     "token_use",
    //     "scope",
    //     "auth_time",
    //     "exp",
    //     "iat",
    //     "username",
    // ]);

    let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let token_data = match decode::<CognitoClaims>(token, &decoding_key, &validation) {
        Ok(data) => {
            info!(sub = %data.claims.sub, "token validated successfully");
            data
        }
        Err(e) => {
            warn!(error = %e, "token validation failed with error");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Additional custom validations.
    if token_data.claims.token_use != "access" {
        return Err(StatusCode::UNAUTHORIZED);
    }
    if token_data.claims.client_id != jwk_manager.get_client_id() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Store claims in request extensions for handlers to access.
    req.extensions_mut().insert(token_data.claims);
    Ok(next.run(req).await)
}

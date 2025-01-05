use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CognitoJWKS {
    pub keys: Vec<Jwk>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwk {
    kid: String,
    kty: String,
    alg: String,
    n: String,
    e: String,
    r#use: String,
}

pub async fn get_jwks(oidc_authority: &str) -> Result<CognitoJWKS> {
    let client = Client::new();
    let jwks: CognitoJWKS = client
        .get(oidc_authority)
        .send()
        .await
        .with_context(|| {
            format!(
                "Error obtaining JWKs for the OIDC authority {}",
                oidc_authority
            )
        })?
        .json()
        .await
        .with_context(|| {
            format!(
                "Error parsing JWKs for the OIDC authority {}",
                oidc_authority
            )
        })?;
    Ok(jwks)
}

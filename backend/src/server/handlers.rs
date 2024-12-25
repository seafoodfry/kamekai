use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response, StatusCode};
use serde_json::json;
use std::convert::Infallible;

use super::models::{BuilderError, LanguageTranslation, Translation, TranslationResponse};

fn create_response(
    status: StatusCode,
    content_type: &str,
    body: impl Into<Bytes>,
) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .header("content-type", content_type)
        .header("x-protocol", "h2")
        .body(Full::new(body.into()))
        .unwrap_or_else(|e| {
            eprintln!("Failed to create response: {}", e);
            Response::new(Full::new(Bytes::from(
                r#"{"error":"Internal Server Error"}"#,
            )))
        })
}

// Returns Result<Response, Infallible> because it handles all its own errors
// by converting them into appropriate HTTP responses.
// We ALWAYS return a Response, never an Err.
pub async fn handle_translate(
    _: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let response = match create_dummy_response() {
        Ok(response) => response,
        Err(e) => {
            // Convert builder error to a proper API error response.
            eprintln!("Failed to create response: {}", e);
            return Ok(create_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "application/json",
                json!({
                    "error": format!("Failed to create response")
                })
                .to_string(),
            ));
        }
    };

    Ok(match serde_json::to_string(&response) {
        Ok(json) => create_response(StatusCode::OK, "application/json", json),
        Err(e) => {
            eprintln!("JSON serialization error: {}", e);
            create_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "application/json",
                json!({
                    "error": "Internal Server Error"
                })
                .to_string(),
            )
        }
    })
}

pub async fn handle_not_found(
    _: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("content-type", "text/plain")
        .body(Full::new(Bytes::from("Not Found")))
        .unwrap())
}

fn create_dummy_response() -> Result<TranslationResponse, BuilderError> {
    let japanese = LanguageTranslation::builder()
        .translation("こんにちは世界".to_string())
        .pronunciation("Konnichiwa sekai".to_string())
        .grammar("Basic greeting + noun".to_string())
        .build()?;

    let chinese = LanguageTranslation::builder()
        .translation("你好世界".to_string())
        .pronunciation("Nǐ hǎo shìjiè".to_string())
        .grammar("Basic greeting + noun".to_string())
        .build()?;

    let translation = Translation::builder()
        .original("Hello world".to_string())
        .japanese(japanese)
        .chinese(chinese)
        .build()?;

    Ok(TranslationResponse::builder()
        .add_translation(translation)
        .build())
}

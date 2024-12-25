use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use super::models::{
    BuilderError, LanguageTranslation, Translation, TranslationRequest, TranslationResponse,
};

pub async fn handle_health() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({ "status": "healthy" }))).into_response()
}

pub async fn handle_translate(Json(payload): Json<TranslationRequest>) -> impl IntoResponse {
    match create_translation_response(&payload.text) {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create translation response: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to create translation response"
                })),
            )
                .into_response()
        }
    }
}

fn create_translation_response(text: &str) -> Result<TranslationResponse, BuilderError> {
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
        .original(text.to_string())
        .japanese(japanese)
        .chinese(chinese)
        .build()?;

    Ok(TranslationResponse::builder()
        .add_translation(translation)
        .build())
}

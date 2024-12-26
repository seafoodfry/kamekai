use anyhow::{Context, Result};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{
    aws,
    conversation::ConversationBuilder,
    server::models::{
        BuilderError, Example, ExampleBuilder, LanguageTranslation, Translation,
        TranslationRequest, TranslationResponse,
    },
};

pub async fn handle_health() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({ "status": "healthy" }))).into_response()
}

pub async fn handle_translate(Json(payload): Json<TranslationRequest>) -> impl IntoResponse {
    match process_translation(&payload.text).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => {
            // Log the full error chain.
            tracing::error!(
                "Failed to process translation. Error chain: \n{:?}",
                e.chain().collect::<Vec<_>>()
            );
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

async fn process_translation(text: &str) -> Result<TranslationResponse> {
    // Initialize the AWS client.
    let aws_client = aws::AWSClient::new(Some(aws::InferenceParameters {
        temperature: 0.8,
        max_tokens: 1024,
        top_p: 0.95,
    }))
    .await
    .context("Error creating AWS client")?;

    let message = ConversationBuilder::new()
    .with_system_prompt(
        r#"You are the brains for an app that aims to teach Japanese and Chinese.
Because you are the brains for an app, you need to respond in JSON format.
The users will send you some text that you need to separate into sentences or phrases,
and then translate them into Japanese and Chinese.
If the text is in english then translate it to Japanese and Chinese.
If the text is in Japanese, then only worry about translating it to Chinese.
If the text is in Chinese, then only worry about translating it to Japanese.

Your response should look like the following example:
{
    "translations": [
        {
            "original": "I told you so",
            "japanese": {
                "translation": "だから言ったでしょう",
                "pronunciation": "dakara itta deshou",
                "grammar": [
                    "だから (dakara): 'That's why' or 'So.'",
                    "言った (いった, itta): The past tense of 言う (いう, iu), meaning 'to say' or 'to tell.'",
                    "でしょう (deshou): A sentence-ending particle that adds a tone of confirmation or assertion, often implying 'didn't I?' or 'right?'"
                ],
                "examples": [
                    {
                        "phrase": "ほら、だから言ったでしょう！",
                        "pronunciation": "ほら、だからいったでしょう!",
                        "translation": "See, I told you so!"
                    },
                    {
                        "phrase": "言ったよね",
                        "pronunciation": "いったよね",
                        "translation": "I told you, right?"
                    }
                ]
            },
            "chinese": {
                "translation": "我早就跟你说了",
                "pronunciation": "wǒ zǎo jiù gēn nǐ shuō le",
                "grammar": [
                    "早就 (zǎo jiù): 'A long time ago' or 'already.'",
                    "跟 (gēn): 'With' or 'to.'",
                    "说了 (shuō le): 'Said' or 'told.'"
                ],
                "examples": [
                    {
                        "phrase": "我就说嘛",
                        "pronunciation": "wǒ jiù shuō ma",
                        "translation": "See, I said so!"
                    },
                    {
                        "phrase": "你看，我不是早就说过了吗！",
                        "pronunciation": "nǐ kàn, wǒ bù shì zǎo jiù shuō guò le ma!",
                        "translation": "See? Didn't I already tell you!"
                    }
                ]
            }
        }
    ]
}"#
    )
    .add_user_message(text)
    .build()
    .context("Error creating messages for AWS Bedrock")?;

    let output = aws_client
        .create_conversation(vec![message])
        .await
        .context("Error creating conversation with AWS Bedrock")?;

    println!("Output:\n{}", output);
    let response: TranslationResponse =
        serde_json::from_str(&output).context("Error parsing Bedrock response")?;

    Ok(response)
}

#[allow(dead_code)]
fn create_translation_response(text: &str) -> Result<TranslationResponse, BuilderError> {
    let japanese_grammar = vec![
        "だから (dakara): 'That's why' or 'So.'",
        "言った (いった, itta): The past tense of 言う (いう, iu), meaning 'to say' or 'to tell.'",
        "でしょう (deshou): A sentence-ending particle that adds a tone of confirmation or assertion, often implying 'didn't I?' or 'right?'"
    ];
    let japanese_examples = vec![
        Example::builder()
            .phrase("ほら、だから言ったでしょう！")
            .pronunciation("ほら、だからいったでしょう!")
            .translation("See, I told you so!")
            .build()?,
        Example::builder()
            .phrase("言ったよね")
            .pronunciation("いったよね")
            .translation("I told you, right?")
            .build()?,
    ];
    let japanese = LanguageTranslation::builder()
        .translation("こんにちは世界".to_string())
        .pronunciation("Konnichiwa sekai".to_string())
        .grammars(japanese_grammar)
        .examples(japanese_examples)
        .build()?;

    let chinese_grammar = vec![
        "早就 (zǎo jiù): 'A long time ago' or 'already.'",
        "跟 (gēn): 'With' or 'to.'",
        "说了 (shuō le): 'Said' or 'told.'",
    ];
    let chinese_examples = vec![
        ExampleBuilder::new()
            .phrase("我就说嘛")
            .pronunciation("wǒ jiù shuō ma")
            .translation("See, I said so!")
            .build()?,
        ExampleBuilder::new()
            .phrase("你看，我不是早就说过了吗！")
            .pronunciation("nǐ kàn, wǒ bù shì zǎo jiù shu")
            .translation("See? Didn't I already tell you!")
            .build()?,
    ];
    let chinese = LanguageTranslation::builder()
        .translation("你好世界".to_string())
        .pronunciation("Nǐ hǎo shìjiè".to_string())
        .grammars(chinese_grammar)
        .examples(chinese_examples)
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

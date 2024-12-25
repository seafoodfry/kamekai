use anyhow::{Context, Result};
use chrono::Local;

pub mod aws;
pub mod conversation;
pub mod error;
pub mod language;
pub mod server;

pub use conversation::builder::ConversationBuilder;
pub use error::AppError;
pub use language::Language;

// High-level function that encapsulates the main conversation flow.
pub async fn create_conversation(language: Language) -> Result<String> {
    // Initialize the AWS client.
    let aws_client = aws::AWSClient::new(Some(aws::InferenceParameters {
        temperature: 0.8,
        max_tokens: 1024,
        top_p: 0.95,
    }));

    // Generate the prompt.
    let greeting = language.get_greeting();
    let prompt = format!(
        "{} Please teach me something interesting in {}.  ",
        greeting, language,
    );

    let current_time = Local::now();
    let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S UTC").to_string();
    let enriched_prompt = format!(
        "Current time: {}. Day of week: {}. {} {}",
        formatted_time,
        current_time.format("%A"),
        greeting,
        prompt
    );

    let message = ConversationBuilder::new()
        .with_system_prompt(format!(
            "You are a {} language teacher who creates unique, contextualized lessons. \
            Each lesson should combine: \
            1. A grammar point with practical examples \
            2. Theme-appropriate vocabulary \
            3. A real-world situation or scenario \
            4. Cultural context when relevant \
            5. Discuss formal and informal language, when appropriate \
            6. Always explain how things are pronounced \
            Vary your teaching style and difficulty level. \
            Sometimes focus on casual speech, sometimes on formal language. \
            Include both basic and advanced concepts.
            And keep in mind that your response will finalize the lesson, the use will not reply.",
            language,
        ))
        .add_user_message(enriched_prompt)
        .build()
        .context("Error creating messages for AWS Bedrock")?;

    aws_client
        .create_conversation(vec![message])
        .await
        .context("Error creating conversation with AWS Bedrock")
}

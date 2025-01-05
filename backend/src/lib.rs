use anyhow::{Context, Result};
use chrono::Local;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    EnvFilter,
};

pub mod aws;
pub mod conversation;
pub mod error;
pub mod language;
pub mod server;

pub use conversation::builder::ConversationBuilder;
pub use error::AppError;
pub use language::Language;

pub fn init_cli_logging() -> Result<()> {
    // Get log level from environment or default to INFO.
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Set up the subscriber with formatting appropriate for CLI.
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        // Show levels (ERROR, WARN, INFO, etc.)
        .with_level(true)
        // Show targets (module paths)
        .with_target(true)
        // Show line numbers
        .with_line_number(true)
        // Use colors in the output
        .with_ansi(true)
        // Format timestamps for human readability
        .with_timer(fmt::time::LocalTime::rfc_3339())
        // Only show spans when they have events
        .with_span_events(FmtSpan::ACTIVE)
        // Pretty printed logging for better CLI readability
        .pretty()
        .try_init()
        .map_err(|e| anyhow::anyhow!(e))?;

    Ok(())
}

// High-level function that encapsulates the main conversation flow.
pub async fn create_conversation(language: Language) -> Result<String> {
    // Initialize logging.
    init_cli_logging()?;

    // Initialize the AWS client.
    let aws_client = aws::AWSClient::new(Some(aws::InferenceParameters {
        temperature: 0.8,
        max_tokens: 1024,
        top_p: 0.95,
    }))
    .await
    .context("Error creating AWS client")?;

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

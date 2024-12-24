use chrono::{DateTime, Timelike, Utc};

pub mod aws;
pub mod conversation;
pub mod error;

// Re-export the most commonly used types and functions ("convenient imports").
// Rustaceans do it. You can do it too.
pub use conversation::builder::ConversationBuilder;
pub use error::AppError;

pub const AWS_REGION: &str = "us-east-1";

fn get_japanese_greeting(time: &DateTime<Utc>) -> &'static str {
    let hour = time.hour();
    match hour {
        5..=10 => "おはようございます。",
        11..=17 => "こんにちは。",
        _ => "こんばんは。",
    }
}

// High-level function that encapsulates the main conversation flow.
pub async fn create_conversation(prompt: &str) -> Result<String, AppError> {
    let sdk_config = aws_config::defaults(aws_config::BehaviorVersion::v2024_03_28())
        .region(AWS_REGION)
        .load()
        .await;
    let client = aws_sdk_bedrockruntime::Client::new(&sdk_config);

    let aws_account_id = aws::sts::get_aws_account_id(&sdk_config).await?;
    let aws_inference_profile = format!(
        "arn:aws:bedrock:us-east-1:{}:inference-profile/us.anthropic.claude-3-5-sonnet-20241022-v2:0",
        aws_account_id
    );

    // Create a timestamp-enriched message.
    let current_time = Utc::now();
    let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S UTC").to_string();
    let greeting = get_japanese_greeting(&current_time);
    println!(
        "Current time: {}. Day of week: {}. Greeting: {}",
        formatted_time,
        current_time.format("%A"),
        greeting
    );

    let enriched_prompt = format!(
        "Current time: {}. Day of week: {}. {} {}",
        formatted_time,
        current_time.format("%A"),
        greeting,
        prompt
    );

    let message = ConversationBuilder::new()
        .with_system_prompt(
            "You are a Japanese language teacher who creates unique, contextualized lessons. \
            Each lesson should combine: \
            1. A grammar point with practical examples \
            2. Theme-appropriate vocabulary \
            3. A real-world situation or scenario \
            4. Cultural context when relevant \
            Vary your teaching style and difficulty level. \
            Sometimes focus on casual speech, sometimes on formal language. \
            Include both basic and advanced concepts.",
        )
        .add_user_message(enriched_prompt)
        .build()?;

    aws::bedrock::send_conversation(&client, &aws_inference_profile, vec![message]).await
}

use crate::error::AppError;
use aws_sdk_bedrockruntime::{
    operation::converse::ConverseOutput,
    types::{InferenceConfiguration, Message},
    Client,
};

pub fn get_converse_output_text(output: ConverseOutput) -> Result<String, AppError> {
    output
        .output()
        .ok_or_else(|| AppError::MessageParse("no output".into()))?
        .as_message()
        .map_err(|_| AppError::MessageParse("output not a message".into()))?
        .content()
        .first()
        .ok_or_else(|| AppError::MessageParse("no content in message".into()))?
        .as_text()
        .map_err(|_| AppError::MessageParse("content is not text".into()))
        .map(|s| s.to_string())
}

pub async fn send_conversation(
    client: &Client,
    aws_inference_profile: &str,
    messages: Vec<Message>,
) -> Result<String, AppError> {
    // Create the inference configuration.
    let inference_config = InferenceConfiguration::builder()
        .temperature(0.8) // Add some randomness but keep responses focused.
        .max_tokens(1024) // Allow for detailed responses.
        .top_p(0.95) // Control diversity of responses.
        .build();

    // Build and send our request.
    let response = client
        .converse()
        .model_id(aws_inference_profile)
        .set_messages(Some(messages))
        .set_inference_config(Some(inference_config))
        .send()
        .await
        .map_err(|e| AppError::Bedrock(format!("{:#?}", e)))?;

    get_converse_output_text(response)
}

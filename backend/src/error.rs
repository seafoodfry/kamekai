use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("AWS Bedrock error: {0}")]
    Bedrock(String),

    #[error("AWS STS error: {0}")]
    Sts(String),

    #[error("Message parsing error: {0}")]
    MessageParse(String),

    #[error("Server error: {0}")]
    Server(String),
}
